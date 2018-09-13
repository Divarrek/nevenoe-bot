# nevenoe-bot

Discord bot that provides access to breton language tools

Usage
=====
 
!help

lists all available commands

!termofis [term]

directly access the termofis dictionnary and print out result in markdown. fr=>br

!glosbe [term]

give result from glosbe dictionnary (fr => br)

!difazi [sentence]

corrects the breton sentence from languagetool API (unstable, doesn't currently work with utf8 characters)

Installation
============

run 'DISCORD_TOKEN=[token] cargo run' in the root directory
