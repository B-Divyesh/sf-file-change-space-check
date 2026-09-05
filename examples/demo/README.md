# Bundled demo sample

`fcsc --demo` creates this sample shape inside a new operating-system temporary
directory. The files represent a media archive with one existing photo, one
new video, one small notes file, and one sparse disk image.

```text
source/
  archive/interview.mov
  archive/project-notes.txt
  disk-images/field-laptop.img
  photos.raw
destination/
  photos.raw
```

The executable creates compact local stand-ins with these names and realistic
allocation differences. It writes only inside the printed temporary sandbox.
