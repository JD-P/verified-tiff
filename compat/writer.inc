/* A deliberately limited writer: serialize image metadata for applications
 * that construct temporary decoder contexts. Image encoding remains ENOSYS. */
int TIFFWriteCheck(TIFF *t,int tiles,const char *module){
  (void)module;if(!valid(t,__func__))return 0;
  if(!vt_write_check(t->state)){bridge_fail(t,__func__);return 0;}
  if(!!tiles!=TIFFIsTiled(t)){fail(t,EINVAL,__func__,"strip/tile layout mismatch");return 0;}return 1;
}
int TIFFWriteDirectory(TIFF *t){
  if(!valid(t,__func__))return 0;
  if(!(t->flags&4)||!t->write||!t->seek){vt_not_implemented_for(t,__func__);return 0;}
  size_t n=0;const uint8_t *p=vt_directory_bytes(t->state,&n);
  if(!p){bridge_fail(t,__func__);return 0;}
  if(t->seek(t->client,0,SEEK_SET)!=0){fail(t,EIO,__func__,"temporary directory seek failed");return 0;}
  size_t at=0;while(at<n){size_t want=n-at;if(want>65536)want=65536;tmsize_t got=t->write(t->client,(void*)(p+at),(tmsize_t)want);if(got<=0||(size_t)got>want){fail(t,EIO,__func__,"temporary directory write failed");return 0;}at+=(size_t)got;}
  t->directory_written=1;return 1;
}
int TIFFFlushData(TIFF *t){return valid(t,__func__);}
int TIFFFlush(TIFF *t){if(!valid(t,__func__))return 0;return (t->flags&4)&&!t->directory_written?TIFFWriteDirectory(t):1;}

int TIFFReadFromUserBuffer(TIFF *t,uint32_t k,void *input,tmsize_t n,void *output,tmsize_t wanted){
  if(!valid(t,__func__))return 0;
  if(n<0||wanted<0||(!input&&n)||(!output&&wanted)){fail(t,EINVAL,__func__,"invalid user buffer");return 0;}
  const uint8_t *p=vt_user_buffer(t->state,k,input,(size_t)n,(size_t)wanted);
  if(!p){bridge_fail(t,__func__);return 0;}
  if(wanted&&!vt_commit(output,(size_t)wanted,0,p,(size_t)wanted)){bridge_fail(t,__func__);return 0;}
  return 1;
}
