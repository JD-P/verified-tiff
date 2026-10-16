/* Public metadata ABI. Field values are converted and bounds checked in Rust.
 * Registration and C varargs handling remain explicitly unverified. */
struct _TIFFField {
  uint32_t tag;
  TIFFDataType type;
  int read_count,write_count,pass_count,size,count_size,custom;
  const char *name;
  TIFFDataType kind;
  int array,anonymous;
};
#include "field_schemas.inc"
struct vt_field_node {struct vt_field_node *next; TIFFField f; unsigned dir; size_t charge; char name[];};
static _Atomic(TIFFExtendProc) tag_extender;
TIFFExtendProc TIFFSetTagExtender(TIFFExtendProc p){return atomic_exchange(&tag_extender,p);}
static const TIFFField *static_field(TIFF *t,uint32_t tag){
  const TIFFField *a=tiffFields;size_t n=sizeof(tiffFields)/sizeof(*a);
  if(t->directory_kind==1){a=exifFields;n=sizeof(exifFields)/sizeof(*a);}
  if(t->directory_kind==2){a=gpsFields;n=sizeof(gpsFields)/sizeof(*a);}
  for(size_t i=0;i<n;i++)if(a[i].tag==tag)return a+i;
  return NULL;
}
static const TIFFField *lookup_field(TIFF *t,uint32_t tag){
  for(struct vt_field_node *p=t->fields;p;p=p->next)
    if(p->dir==t->directory_kind&&p->f.tag==tag)return &p->f;
  return static_field(t,tag);
}
static int add_field(TIFF *t,TIFFField f){
  if(t->field_count>=4096){fail(t,EFBIG,"TIFFMergeFieldInfo","field registration limit");t->field_error=1;return 0;}
  size_t n=strnlen(f.name,4096);if(n==4096){fail(t,EINVAL,"TIFFMergeFieldInfo","field name too long");t->field_error=1;return 0;}
  size_t charge=sizeof(struct vt_field_node)+n+1;
  if(!vt_mem_claim(t->state,charge)){bridge_fail(t,"TIFFMergeFieldInfo");t->field_error=1;return 0;}
  struct vt_field_node *p=malloc(charge);
  if(!p){vt_mem_release(t->state,charge);fail(t,ENOMEM,"TIFFMergeFieldInfo","allocation failed");t->field_error=1;return 0;}
  memcpy(p->name,f.name,n+1);p->f=f;p->f.name=p->name;p->dir=t->directory_kind;p->charge=charge;
  p->next=t->fields;t->fields=p;t->field_count++;return 1;
}
static void free_fields(TIFF *t){
  while(t->fields){struct vt_field_node *p=t->fields;t->fields=p->next;vt_mem_release(t->state,p->charge);free(p);}t->field_count=0;
}
static TIFFDataType host_type(TIFFDataType type){
  return (TIFFDataType)vt_host_type((uint16_t)type);
}
int TIFFMergeFieldInfo(TIFF *t,const TIFFFieldInfo info[],uint32_t n){
  if(!valid(t,__func__))return -1;
  struct vt_field_node *head=t->fields;
  if((!info&&n)||n>4096){fail(t,EINVAL,__func__,"invalid field array");goto rollback;}
  for(uint32_t i=0;i<n;i++){
    const TIFFFieldInfo *x=info+i;
    if(lookup_field(t,x->field_tag))continue;
    TIFFDataType kind=host_type(x->field_type);int size=TIFFDataWidth(kind);
    if(!x->field_name||!size||x->field_tag>65535||!vt_field_registration((uint16_t)x->field_type,x->field_readcount,x->field_writecount,x->field_passcount)){fail(t,EINVAL,__func__,"invalid field definition");goto rollback;}
    TIFFField f={x->field_tag,x->field_type,x->field_readcount,x->field_writecount,x->field_passcount,size,x->field_passcount?(x->field_readcount==TIFF_VARIABLE2?4:2):0,x->field_bit==FIELD_CUSTOM,x->field_name,kind,x->field_passcount||x->field_readcount!=1||kind==TIFF_ASCII,0};
    if(!add_field(t,f))goto rollback;
  }
  return 0; /* libtiff's legacy registration API returns zero on success. */
rollback:
  while(t->fields!=head){struct vt_field_node *p=t->fields;t->fields=p->next;t->field_count--;vt_mem_release(t->state,p->charge);free(p);}
  t->field_error=1;return -1;
}
static const TIFFField *resolve_field(TIFF *t,uint32_t tag){
  const TIFFField *f=lookup_field(t,tag);if(f)return f;
  uint16_t kind=0;size_t n=0;
  if(tag>65535||!vt_field(t->state,tag,&kind,&n))return NULL;
  char name[32];snprintf(name,sizeof(name),"Tag %u",tag);
  TIFFDataType host=host_type((TIFFDataType)kind);
  TIFFField v={tag,(TIFFDataType)kind,-3,-3,1,TIFFDataWidth(host),4,1,name,host,1,1};
  if(!add_field(t,v))return NULL;
  return &t->fields->f;
}
static int initialise_fields(TIFF *t,unsigned kind){
  t->directory_kind=kind;t->field_error=0;
  if(kind==0){t->methods.vgetfield=default_getfield;t->methods.vsetfield=default_setfield;t->methods.printdir=NULL;TIFFExtendProc f=atomic_load(&tag_extender);if(f)f(t);}
  return !t->field_error;
}
const TIFFField *TIFFFindField(TIFF *t,uint32_t tag,TIFFDataType type){
  if(!valid(t,__func__))return NULL;
  const TIFFField *f=resolve_field(t,tag);
  return f&&(type==TIFF_ANY||type==f->type)?f:NULL;
}
const TIFFField *TIFFFieldWithTag(TIFF *t,uint32_t tag){return TIFFFindField(t,tag,TIFF_ANY);}
const TIFFField *TIFFFieldWithName(TIFF *t,const char *name){
  if(!valid(t,__func__)||!name)return NULL;
  for(struct vt_field_node *p=t->fields;p;p=p->next)if(p->dir==t->directory_kind&&!strcmp(p->f.name,name))return &p->f;
  const TIFFField *a=tiffFields;size_t n=sizeof(tiffFields)/sizeof(*a);
  if(t->directory_kind==1){a=exifFields;n=sizeof(exifFields)/sizeof(*a);}
  if(t->directory_kind==2){a=gpsFields;n=sizeof(gpsFields)/sizeof(*a);}
  for(size_t i=0;i<n;i++)if(!strcmp(a[i].name,name))return a+i;
  return NULL;
}
uint32_t TIFFFieldTag(const TIFFField *f){return f?f->tag:0;}
const char *TIFFFieldName(const TIFFField *f){return f?f->name:NULL;}
TIFFDataType TIFFFieldDataType(const TIFFField *f){return f?f->type:TIFF_NOTYPE;}
int TIFFFieldPassCount(const TIFFField *f){return f?f->pass_count:0;}
int TIFFFieldReadCount(const TIFFField *f){return f?f->read_count:0;}
int TIFFFieldWriteCount(const TIFFField *f){return f?f->write_count:0;}
int TIFFFieldSetGetSize(const TIFFField *f){return f?f->size:0;}
int TIFFFieldSetGetCountSize(const TIFFField *f){return f?f->count_size:0;}
int TIFFFieldIsAnonymous(const TIFFField *f){return f?f->anonymous:0;}
int TIFFGetTagListCount(TIFF *t){
  if(!valid(t,__func__))return 0;
  int count=0;size_t n=info(t,15);
  for(size_t i=0;i<n;i++){const TIFFField *f=resolve_field(t,vt_tag(t->state,i));if(f&&f->custom)count++;}return count;
}
uint32_t TIFFGetTagListEntry(TIFF *t,int index){
  if(!valid(t,__func__)||index<0)return UINT32_MAX;
  size_t n=info(t,15);
  for(size_t i=0;i<n;i++){uint32_t tag=vt_tag(t->state,i);const TIFFField *f=resolve_field(t,tag);if(f&&f->custom&&index--==0)return tag;}return UINT32_MAX;
}
static int transfer_getfield(TIFF *t,va_list ap,int def){
  uint16_t bits=1,ch=1,extras=0,*extra=NULL;
  TIFFGetFieldDefaulted(t,TIFFTAG_BITSPERSAMPLE,&bits);TIFFGetFieldDefaulted(t,TIFFTAG_SAMPLESPERPIXEL,&ch);TIFFGetFieldDefaulted(t,TIFFTAG_EXTRASAMPLES,&extras,&extra);
  if(bits==0||bits>16||extras>ch){fail(t,EINVAL,"TIFFGetField","transfer function geometry");return 0;}
  const uint8_t *data=NULL;size_t n=0,curve=(size_t)1<<bits;int many=ch-extras>1;
  int r=vt_typed(t->state,TIFFTAG_TRANSFERFUNCTION,TIFF_SHORT,&data,&n);
  if(r<0){bridge_fail(t,"TIFFGetField");return 0;}
  if(!r){
    if(!def)return 0;
    if(t->transfer_count!=curve){
      size_t bytes=curve*sizeof(uint16_t);if(!vt_mem_claim(t->state,bytes)){bridge_fail(t,"TIFFGetFieldDefaulted");return 0;}
      uint16_t *p=malloc(bytes);if(!p){vt_mem_release(t->state,bytes);fail(t,ENOMEM,"TIFFGetFieldDefaulted","transfer table allocation");return 0;}
      for(size_t i=0;i<curve;i++)p[i]=(uint16_t)floor(65535.*pow((double)i/(double)(curve-1),2.2)+0.5);
      if(t->default_transfer){vt_mem_release(t->state,t->transfer_count*sizeof(uint16_t));free(t->default_transfer);}
      t->default_transfer=p;t->transfer_count=curve;
    }
    data=(const uint8_t*)t->default_transfer;n=curve;
  }
  if(n!=curve&&!(many&&n==3*curve)){fail(t,EINVAL,"TIFFGetField","transfer table length");return 0;}
  uint16_t **a=va_arg(ap,uint16_t**),**b=NULL,**c=NULL;if(many){b=va_arg(ap,uint16_t**);c=va_arg(ap,uint16_t**);}
  if(!a||(many&&(!b||!c)))return 0;
  *a=(uint16_t*)data;if(many){*b=(uint16_t*)data+(n==curve?0:curve);*c=(uint16_t*)data+(n==curve?0:2*curve);}return 1;
}
static int generic_getfield(TIFF *t,uint32_t tag,va_list ap,int def){
  if(!t->directory_kind&&tag==TIFFTAG_TRANSFERFUNCTION)return transfer_getfield(t,ap,def);
  const TIFFField *f=resolve_field(t,tag);
  if(!f)return 0; /* Optional absent tags do not raise a decode error. */
  const uint8_t *data=NULL;size_t n=0;
  TIFFDataType kind=f->kind;
  if(!f->size){uint16_t stored=0;vt_field(t->state,tag,&stored,&n);if(!n)return 0;fail(t,ENOSYS,"TIFFGetField","special field convention not implemented");return 0;}
  int r=vt_typed(t->state,tag,(uint16_t)kind,&data,&n);
  if(r<0){bridge_fail(t,"TIFFGetField");return 0;}
  static const double one=1.0;
  static const float white[]={D50_X0/(D50_X0+D50_Y0+D50_Z0),D50_Y0/(D50_X0+D50_Y0+D50_Z0)};
  static const float luma[]={0.299f,0.587f,0.114f};
  if(!r&&def&&tag==TIFFTAG_WHITEPOINT){data=(const uint8_t*)white;n=2;r=1;}
  if(!r&&def&&tag==TIFFTAG_YCBCRCOEFFICIENTS){data=(const uint8_t*)luma;n=3;r=1;}
  if(!r&&def&&tag==TIFFTAG_REFERENCEBLACKWHITE){
    uint16_t bits=1,photo=1;TIFFGetFieldDefaulted(t,TIFFTAG_BITSPERSAMPLE,&bits);TIFFGetFieldDefaulted(t,TIFFTAG_PHOTOMETRIC,&photo);
    float white=(float)(pow(2.,bits)-1.);for(int i=0;i<6;i++)t->default_reference[i]=(i%2)?white:0.;
    if(photo==PHOTOMETRIC_YCBCR){t->default_reference[1]=t->default_reference[3]=t->default_reference[5]=255.;t->default_reference[2]=t->default_reference[4]=128.;}
    data=(const uint8_t*)t->default_reference;n=6;r=1;
  }
  if(!r&&def&&tag==TIFFTAG_STONITS){data=(const uint8_t*)&one;n=1;r=1;}
  if(!r)return 0;
  unsigned plan=vt_field_plan((uint16_t)kind,f->read_count,f->pass_count,(size_t)f->count_size,f->array,(size_t)f->size,n);
  if(!plan){fail(t,EINVAL,"TIFFGetField","field schema/count mismatch");return 0;}
  if(plan==3||plan==4){
    uint32_t *c32=NULL;uint16_t *c16=NULL;
    if(plan==4)c32=va_arg(ap,uint32_t*);else c16=va_arg(ap,uint16_t*);
    void **out=va_arg(ap,void **);if((plan==4?!c32:!c16)||!out)return 0;
    if(plan==4)*c32=(uint32_t)n;else *c16=(uint16_t)n;*out=(void*)data;
  }else if(plan==2){void **out=va_arg(ap,void **);if(!out)return 0;*out=(void*)data;}
  else{
#define VT_SCALAR(T) do{T *out=va_arg(ap,T*);if(!out)return 0;memcpy(out,data,sizeof(T));}while(0)
    switch(kind){
      case TIFF_BYTE:case TIFF_UNDEFINED:VT_SCALAR(uint8_t);break;
      case TIFF_SBYTE:VT_SCALAR(int8_t);break;
      case TIFF_SHORT:VT_SCALAR(uint16_t);break;
      case TIFF_SSHORT:VT_SCALAR(int16_t);break;
      case TIFF_LONG:case TIFF_IFD:VT_SCALAR(uint32_t);break;
      case TIFF_SLONG:VT_SCALAR(int32_t);break;
      case TIFF_LONG8:case TIFF_IFD8:VT_SCALAR(uint64_t);break;
      case TIFF_SLONG8:VT_SCALAR(int64_t);break;
      case TIFF_FLOAT:VT_SCALAR(float);break;
      case TIFF_DOUBLE:VT_SCALAR(double);break;
      default:return 0;
    }
#undef VT_SCALAR
  }
  return 1;
}
static void unknown_warnings(TIFF *t){
  if(!t||!t->options.warn_unknown)return;
  size_t n=info(t,15);
  for(size_t i=0;i<n;i++){uint32_t tag=vt_tag(t->state,i);if(!lookup_field(t,tag))TIFFWarningExtR(t,"TIFFReadDirectory","Unknown field with tag %u",tag);}
}
int TIFFReadEXIFDirectory(TIFF *t,uint64_t off){
  if(!valid(t,__func__))return 0;
  if(!vt_custom_dir(t->state,off,1)){bridge_fail(t,__func__);return 0;}return initialise_fields(t,1);
}
int TIFFReadGPSDirectory(TIFF *t,uint64_t off){
  if(!valid(t,__func__))return 0;
  if(!vt_custom_dir(t->state,off,2)){bridge_fail(t,__func__);return 0;}return initialise_fields(t,2);
}
static int generic_setfield(TIFF *t,uint32_t tag,va_list ap){
  const TIFFField *f=lookup_field(t,tag);
  if(!f){fail(t,ENOSYS,"TIFFSetField","unregistered field");return 0;}
  TIFFDataType kind=f->kind;size_t count=1;const void *input=NULL;
  union {uint64_t u;int64_t i;double d;float f;uint32_t u32;int32_t i32;uint16_t u16;int16_t i16;uint8_t u8;int8_t i8;} scalar;
  uint16_t pair[2];
  if(tag==TIFFTAG_PAGENUMBER||tag==TIFFTAG_YCBCRSUBSAMPLING||tag==TIFFTAG_HALFTONEHINTS||tag==TIFFTAG_DOTRANGE){pair[0]=(uint16_t)va_arg(ap,int);pair[1]=(uint16_t)va_arg(ap,int);count=2;kind=TIFF_SHORT;input=pair;}
  else if(tag==TIFFTAG_COLORMAP){
    uint16_t bits=1;TIFFGetFieldDefaulted(t,TIFFTAG_BITSPERSAMPLE,&bits);if(bits>16){fail(t,EINVAL,"TIFFSetField","palette bit depth");return 0;}
    size_t n=(size_t)1<<bits,bytes=n*6;const uint16_t *a=va_arg(ap,const uint16_t*),*b=va_arg(ap,const uint16_t*),*c=va_arg(ap,const uint16_t*);
    if(!a||!b||!c)return 0;
    if(!vt_mem_claim(t->state,bytes)){bridge_fail(t,"TIFFSetField");return 0;}
    uint16_t *p=malloc(bytes);if(!p){vt_mem_release(t->state,bytes);fail(t,ENOMEM,"TIFFSetField","palette allocation");return 0;}
    memcpy(p,a,n*2);memcpy(p+n,b,n*2);memcpy(p+2*n,c,n*2);int ok=vt_put(t->state,tag,TIFF_SHORT,n*3,(const uint8_t*)p);free(p);vt_mem_release(t->state,bytes);if(!ok)bridge_fail(t,"TIFFSetField");else t->directory_written=0;return ok;
  }
  else if(f->pass_count){count=f->write_count==TIFF_VARIABLE2?va_arg(ap,uint32_t):(uint16_t)va_arg(ap,int);input=va_arg(ap,const void*);}
  else if(kind==TIFF_ASCII){input=va_arg(ap,const char*);if(!input)return 0;count=strnlen(input,t->options.output);if(count==t->options.output){fail(t,EFBIG,"TIFFSetField","string limit");return 0;}count++;}
  else if(f->array||f->write_count>1){if(f->write_count<1){fail(t,ENOSYS,"TIFFSetField","unsized array convention");return 0;}count=(size_t)f->write_count;input=va_arg(ap,const void*);}
  else{
    switch(kind){
    case TIFF_BYTE:case TIFF_UNDEFINED:scalar.u8=(uint8_t)va_arg(ap,int);input=&scalar.u8;break;
    case TIFF_SBYTE:scalar.i8=(int8_t)va_arg(ap,int);input=&scalar.i8;break;
    case TIFF_SHORT:scalar.u16=(uint16_t)va_arg(ap,int);input=&scalar.u16;break;
    case TIFF_SSHORT:scalar.i16=(int16_t)va_arg(ap,int);input=&scalar.i16;break;
    case TIFF_LONG:case TIFF_IFD:scalar.u32=va_arg(ap,uint32_t);input=&scalar.u32;break;
    case TIFF_SLONG:scalar.i32=va_arg(ap,int32_t);input=&scalar.i32;break;
    case TIFF_LONG8:case TIFF_IFD8:scalar.u=va_arg(ap,uint64_t);input=&scalar.u;break;
    case TIFF_SLONG8:scalar.i=va_arg(ap,int64_t);input=&scalar.i;break;
    case TIFF_FLOAT:scalar.f=(float)va_arg(ap,double);input=&scalar.f;break;
    case TIFF_DOUBLE:scalar.d=va_arg(ap,double);input=&scalar.d;break;
    default:fail(t,ENOSYS,"TIFFSetField","field storage convention");return 0;
    }
  }
  if(count&&!input){fail(t,EINVAL,"TIFFSetField","null field input");return 0;}
  int ok=(f->type==TIFF_RATIONAL||f->type==TIFF_SRATIONAL)?vt_put_real(t->state,tag,(uint16_t)f->type,(uint16_t)kind,count,input):vt_put(t->state,tag,(uint16_t)kind,count,input);
  if(!ok)bridge_fail(t,"TIFFSetField");else t->directory_written=0;return ok;
}
