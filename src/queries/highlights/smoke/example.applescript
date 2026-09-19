-- café: tell return are comments
(* outer (* nested *) "*)" still comment *)
property greeting : "Hello"
on greet(personName)
	set message to "café \"quoted\" tell -- literal" & personName
	if true then
		return message
	else
		return missing value
	end if
end greet
to doubleValue(inputValue)
	return inputValue * 2
end doubleValue
set |odd name| to 42
set target to alias "Macintosh HD:Users:"
set propertyValue to alias of target
copy 1.5 to amount
set personRecord to {name:"Ada", age:36, active:true}
repeat with i from 1 to 3
	if i > 1 then return false
end repeat
tell application "Finder"
	set firstName to name of item 1 of home
end tell
try
	display dialog "Hello" default answer ¬
		"world"
on error errMsg number errNum
	log errMsg & errNum
end try
set resultValue to my greet("World")
SeT afterValue to 99
