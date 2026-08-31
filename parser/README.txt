# Grammar

Right now 1 file is 1 sfol sexpr
sfol is a subset of all sexpr

<file>    := <form>
<form>    := <atom>
              | "(" <predicate> <term>* ")"
              | "(" <unary-op> <form> ")"
              | "(" <binary-op> <form> <form> ")"
              | "(" <binary-op+> <form> <form>+ ")"
              | "(" <quantifier> <var>+ <form> ")"
              | "(" "eq" <term> <term> ")"

<unary-op>   := "not"
<binary-op>  := "implies"
<binary-op+>  := "and" | "or" 
<quantifier> := "exists" | "forall"

<term>       := <var>
<var>        := <atom>
<predicate>  := <atom>
<atom>       := any utf-8 string without <excluded>

Extra:
<excluded>   := "(" | ")" | ";;" | " "
