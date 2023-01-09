# qwiki 
qwiki (pronounced KWIK-EE) is a command-line tool for viewing Wikipedia articles on the web. 

## Installation
To install this repository, type `git clone https://github.com/KedarPanchal/qwiki.git` into your terminal.  
To build qwiki, have `cargo` installed on your system. Type `cargo install qwiki` to install qwiki.

## Using qwiki 
Type `qwiki <ARTICLE_NAME>` or `qwiki -s <ARTICLE_NAME>` in the command line to get the summary of a Wikipedia article.   
Type `qwiki <ARTICLE_NAME> <SECTION_NAME>` or `qwiki -s <ARTICLE_NAME> <SECTION_NAME>` in the command line to get the content of a section of a Wikipedia article (section names are case sensitive).   
Type `qwiki -t <ARTICLE_NAME>` to get a list of sections for a Wikipedia article.   
Type `qwiki -r <ARTICLE_NAME>` to get a list of the references for a Wikipedia article.  
Type `qwiki -c <ARTICLE_NAME>` to get all of the categories a Wikipedia article belongs to.  
Type `qwiki -l <ARTICLE_NAME>` to get a link to the Wikipedia article.  
Type `qwiki -p <ARTICLE_NAME>` to get the pageid for a Wikipedia article.  
Type `qwiki -v` to get the current version of qwiki.  
Type `qwiki -h` for help.  
