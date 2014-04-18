/* naklo on the river notec!! */

#include <stdio.h> 
#include <stdlib.h> 
#include <FLAC/metadata.h>

/* naklo is a batch metadata writer for FLAC files. It calls on 
 * the FLAC C API (and by extension libFLAC) to do its work. */



/* grab_tags_file() tries to read the tags file in. It returns a 
 * pointer to the file if it is able. It doesn't worry if it is 
 * unable to return a proper pointer; null cases are handled in 
 * main() [ "if (!tagfile) ... "]. */ 
FILE *grab_tags_file(){ 
    FILE *tagfile; 
    /* default check: tag file named just "tags." */
    tagfile = fopen ("tags", "r"); 

    /* no luck? look for other possible names. */
    if (!tagfile) { 
        tagfile = fopen ("tags.txt", "r"); 
    }
    if (!tagfile) { 
        tagfile = fopen ("Tags", "r"); 
    }
    if (!tagfile) { 
        tagfile = fopen ("Tags.txt", "r"); 
    }
    if (!tagfile) { 
        tagfile = fopen ("TAGS", "r"); 
    }
    if (!tagfile) { 
        tagfile = fopen ("TAGS.txt", "r"); 
    }

    return tagfile; 

}



/* the main function; it does all the heavy lifting. */
int main() { 
    
    /* tagfile is ... the tag file. */
    FILE *tagfile; 
    /* currline will be used to store each line of the tag file,
     * one by one. */
    //char *currline; 
    /* rele is an int specifying a "reasonable length" that is 
     * not really context-specific.*/
    //unsigned int rele = 520; 
    
    /* attempt to read tags file */
    tagfile = grab_tags_file(); 

    /* error out? */
    if (!tagfile) { 
        fprintf (stderr, "naklo couldn't find a tag file. Have a look at the help file, maybe? \n"); 
        return 13; 
    } 

    /* passing the above if(!tagfile) means there is an open 
     * file. We should close it before returning. */
    fclose (tagfile); 

    return 0;

} 

