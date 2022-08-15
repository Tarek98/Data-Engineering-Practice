/* Reference: https://docs.scala-lang.org/tutorials/scala-for-java-programmers.html# */

// Scala is a pure object-oriented language in the sense that everything is an object, including numbers or functions.
// It differs from Java in that respect, since Java distinguishes primitive types (e.g. boolean & int) from reference types.
// Ability to manipulate functions as values is a cornerstone of functional programming.

// To compile, we use scalac, the Scala compiler. 
// scalac works like most compilers: 
// -> it takes a source file as argument, maybe some options, and produces one or several object files. 
// -> The object files it produces are standard "Java" class files.

// One of Scala’s strengths is that it makes it very easy to interact with Java code.
import java.util.{Date, Locale}
import java.text.DateFormat._
// It's also possible to inherit from Java classes and implement Java interfaces directly in Scala.

// Singleton object: class with a single instance created only the first time it is used.
object Main extends App  {
    // An environment is a function which associates a value to a (variable) name
    // e.g. { case "x" => 5 }
    type Environment = String => Int

    val now = new Date
    val dateFormat = getDateInstance(LONG, Locale.UK)
    // Infix syntax: for methods of 1 argument: abbreviates: dateFormat.format(now)
    println(dateFormat format now)
    
    /* oncePerSecond(() => 
        println("time flies...")) */
    
    val c = new ComplexNumber(1.5, 3.2)
    println(s"\n\nComplex form of c = ${c.toString()}")
    println(s"Imaginary part of c = ${c.iPart} \n\n")

    val expr: ExprTree = Sum(Sum(Var("x"),Var("x")),Sum(Var("y"),Const(7)))
    val env: Environment = { case "x" => 5 case "y" => 7 }
    println(s"Expression:\n\t$expr\n")
    println(s"Expression Evaluation with (x=5, y=7): \n\t${evaluateExpr(expr, env)}\n")
    println(s"Expression Derivative relative to x:\n\t${deriveExpr(expr, "x")}\n")
    println(s"Expression Derivative relative to y:\n\t${deriveExpr(expr, "y")}\n\n")
    
    val d1 = new CustomDate(2021, 10, 25)
    val d2 = new CustomDate(2021, 11, 24)
    print(s"Date1: $d1\n")
    print(s"Date2: $d2\n")
    print(s"Date1 <= Date2: ${d1 <= d2}\n\n")

    val cell = new Reference[Int]
    var defaultInt: Int = _
    print(s"Default value of Reference[Int] = ${cell.get}\n")
    print(s"Default value of a Scala Int = ${defaultInt}\n")
    cell.set(15)
    println(s"Dereferenced cell value * 2 = ${cell.get * 2}\n\n")
    
    // Does not return a value; return type is declared as Unit.
    def oncePerSecond(callback: () => Unit): Unit = {
        while (true) { callback(); Thread sleep 1000 }
    }
    
    // This evaluation function works by performing pattern matching on the tree "t".
    def evaluateExpr(t: ExprTree, env: Environment): Int = {
        t match {
            case Sum(l,r) => evaluateExpr(l,env) + evaluateExpr(r,env)
            case Var(n) => env(n)
            case Const(v) => v
        }
    } 

    // This derivative function works by performing pattern matching on the tree "t".
    // The parameter "v" is the derivation variable.
    def deriveExpr(t: ExprTree, v: String): ExprTree = {
        t match {
            case Sum(l,r) => Sum(deriveExpr(l,v), deriveExpr(r,v))
            case Var(n) if (v == n) => Const(1)
            case _ => Const(0)
        }
    } 
}

// Basic Scala Class
class ComplexNumber(real: Double, imaginary: Double) {
    // Getter functions without arguments (accessed similar to fields)
    def rPart = real
    def iPart = imaginary

    override def toString() =
        "" + rPart + (if (iPart >= 0) "+" else "") + iPart + "i"
}

// Scala Case Class Example: Math Expression Tree: declaration of abstract super-class and case sub-classes that extend it 
abstract class ExprTree
case class Sum(l: ExprTree, r: ExprTree) extends ExprTree
case class Var(n: String) extends ExprTree
case class Const(v: Int) extends ExprTree

// Apart from inheriting code from a super-class, a Scala class can also import code from one or several traits.
// When a class inherits from a trait, it implements that trait’s interface, and inherits all the code contained in the trait.
trait Ord {
    def <  (that: Any): Boolean
    def <= (that: Any): Boolean = (this < that) || (this == that)
    def >  (that: Any): Boolean = !(this <= that)
    def >= (that: Any): Boolean = !(this < that)
}
class CustomDate(y: Int, m: Int, d: Int) extends Ord {
    def year = y
    def month = m
    def day = d

    override def toString(): String = s"$year-$month-$day"

    override def equals(that: Any): Boolean = {
        that.isInstanceOf[CustomDate] && {
            val other = that.asInstanceOf[CustomDate]
            other.day == day && other.month == month && other.year == year
        }
    }

    override def < (that: Any): Boolean = {
        if (!that.isInstanceOf[CustomDate])
            sys.error("cannot compare " + that + " and a Date")

        val other = that.asInstanceOf[CustomDate]
        
        (year < other.year) ||
            (year == other.year && (month < other.month ||
                (month == other.month && day < other.day)))
    }
}

// Genericity is the ability to write code parametrized by types.
// -> Java programmers resort to using Object, which is the super-type of all objects.
// -> Java's solution doesn’t work for basic types (int, long, float, etc.) and implies programming a lot of dynamic type casts.
// -> Scala makes it possible to define generic classes (and methods) to solve this problem.
// -> ^See first comment block in this file; "Scala is a pure object-oriented language in the sense that everything is an object" etc.
class Reference[T] {
    // "_" represents the default value of an object of type T.
    private var contents: T = _

    def set(value: T): Unit = { contents = value }

    def get: T = contents
}
