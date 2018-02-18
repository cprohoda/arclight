# arclight

-- Very incomplete --

** Language philosophy **
- metadata is the one true promised land
- rapid prototyping via default behavior
- transition from rapid to maintainable through recursive refinement

** Universal Methods **
<	returns preceeding after operation based on following
.	returns following from preceeding instance
:	returns following from preceeding uninstantiated object	 

** Some methods by convention **
:help	
:methods	return public methods
:vars	return public variables
:vars:varname	return info on variable varname 

** Special **
"	quote seperator
(	parenthesis seperator
<	as method, send result to instance
.	as method, return component of instance
:	as method, return component of uninstantiated object
\	escape character (inside string it is a single character & outside string it is a single token)
 	token seperator
\t	branch depth specifier
\n	branch separator

** Todo **
-Write language
-Write ide