/* naklo on the river notec!! */

#include <stdio.h> 
#include <stdlib.h> 
#include <FLAC/metadata.h>

/* naklo is a batch metadata writer for FLAC files. It calls on 
 * the FLAC C API (and by extension libFLAC) to do its work. */


/* grab_tags_file() tries to read the tags file in. It returns a 
 * pointer to the file if it is able. It doesn't worry if it is 
 * unable to return a proper pointer; null cases are handled in 
 * main() [ "if (!tagfile) ... "]. 
 * REMEMBER: call fclose() on the resulting pointer afterwards
 * provided it is not null. */ 
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


/* is_meaningless() takes a character buffer in and determines if 
 * the line is anything more than a blank line. It is called by 
 * the main function to weed out (meaningless for our purposes) 
 * newlines. 
 * is_meaningless() returns an int determining true or false, 
 * 0 being false and 1 being true. It returns 0 if the character
 * buffer is _not_ meaningless, and returns 1 if it is meaningless
 * (i.e. a plain ol' blank line). */
int is_meaningless(char *line) { 
    unsigned int i = 0; 
    while (*(line+i)) { 
        if ((' ' != *(line+i))&&('\t' != *(line+i))) { 
            if ('\b' == *(line+i)) { return 1; } 
            if ('\f' == *(line+i)) { return 1; } 
            if ('\v' == *(line+i)) { return 1; } 
            //if ('\'' == *(line+i)) { return 1; } 
            //if ('\"' == *(line+i)) { return 1; } 
            if ('\n' == *(line+i)) { return 1; }
            /* not space, not tab, not special char, ergo not meaningless */
            return 0; 
        }
        ++i; 
    } 
    /* all spaces...*/
    return 1; 
}


/* find_comment_name() takes a character buffer in and attempts 
 * to determine what Vorbis tag the buffer specifies, if any. 
 * It returns the integer length of the tag (defined as the 
 * continuous length of the first substring encountered in 
 * *line) if one turns up. 
 * For example, "% artist" as an argument returns "6." Additionally,
 * "%    artist" also returns 6, as does "%   artist hhuehue."
 * The above examples show the expected behavior of find_comment_
 * name(), which should halt its search on the first non-applicable 
 * character. Vorbis tags accomodate neither spaces nor newlines, 
 * and this function reflects that in its behavior. 
 * If find_comment_name() returns 0, there was an error, usually 
 * that the line in question is not supposed to be a comment field,
 * that the user forgot to put in a comment field at all, or some
 * other general error. */ 
int find_comment_name(char *line) { 
    unsigned int i,j; 
    i = j = 0; 

    /* remove prepended spaces, if any */
    while (*(line+i)) { 
        if (' ' != *(line+i)) { break; } 
        ++i; 
    } 

    /* look for "%" char signifying comment field */
    if ('%' != *(line+i)) { 
        fprintf(stderr, "complaint: expected tag name.\n");
        return 0; 
    } 

    return j; 
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
     * file courtesy of grab_tags_file(). We should close it 
     * before returning. */
    fclose (tagfile); 

    return 0;

} 

