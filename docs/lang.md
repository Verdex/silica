# silica language features

## functions

```
fun name <parameter*> = <expr> ;
```

## statements

### let

```
let x = <expr>;
```

### **spec**

Types of symbols can be explicitly defined using the spec statement.

```
spec name = <type> ;
spec name<a,b,c> = <type> ;
```

## expressions

### function call

### tuple

```
(<expr>[,]*)
```

### match

```
match <expr> {
    <pattern> => <expr>, *
}
```

### block

```
{ 
    <statement>;*
    <expr>
}
```

### object constructor

```
Constructor
Constructor(<expr>[,]*)
```

## types

### type 

type declarations: 

```
type name = <type>;
type name<a,b,c> = <type>;
```

```
<type> = <generic type : lower case symbol>
       | <concrete type : upper case symbol>
       | <type> -> <type>
       | (<type>[,]*)
       | concrete_type<type[,]*>
```

### generic vs concrete 

Concrete types need to be upper case while generic types are lower case.

## data

```
data name[<generic_type_list>] = <constructor_list>+ ;

constructor = UpperCaseSymbol
            | UpperCaseSymbol(type_list)
```