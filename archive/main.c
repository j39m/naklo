/* naklo on the river notec!! */

#include <stdio.h> 
#include <stdlib.h> 
#include <unistd.h>
#include <string.h>
#include <FLAC/metadata.h>

/* naklo is a batch metadata writer for FLAC files. It calls on 
 * the FLAC C API (and by extension libFLAC) to do its work. */


int main(int argc, char **argv) { 

  FILE *tag_file, *muselist; // tag file and list of music files

  unsigned int klaus; 
  int t, l; t = l = 0; 
  while((klaus = getopt(argc, argv, "t:l:h::")) != -1) {
    switch(klaus) { 
      case 'h': 
        printf("Usage: %s -t <tag file> -l <list of files>\n", argv[0]); 
        return 26; 
      case 't': 
        tag_file = fopen(optarg, "r"); 
        if (!tag_file) { 
          fprintf(stderr, "Problem reading tag file. Abort.\n"); 
          return 13; 
        } 
        t = 1; break; 
      case 'l': 
        muselist = fopen(optarg, "r"); 
        if (!muselist) { 
          fprintf(stderr, "Problem reading list of music files. Abort.\n"); 
          return 13; 
        } 
        l = 1; break; 
      case '?': 
        break; 
      default: 
        break; 
    } 
  } 

  if (!(t && l)) { 
    printf("Usage: %s -t <tag file> -l <list of files>\n", argv[0]); 
    return 26; 
  } 
    
  /* cleanup */
  fclose(tag_file); 
  fclose(muselist); 

  return 0;

} 

