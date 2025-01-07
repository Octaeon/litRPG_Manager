# LitRPG Manager

This is a simple app I made that has one goal - to parse the contents of a file, execute the commands within, and spit out a nicely formatted output file.

This has one goal - to simplify the management of litRPG novels in which an author has to keep track of a RPG-like status in which the values often depend on each other (a character gains 2 points of strength each level, so their strength is their initial value + level * 2).

## Use case

Using this tool will hopefully simplify the process of creating a novel like that. From having to keep track of when something happens, what changes, what are the relations between the various numbers, one can simply write in commands into the code (add 5 to strength), and as the manager parses the contents, it will update its' internal value, and display the updated value when prompted.

Of course, this is a project I had just started creating, so it's very much unfinished.