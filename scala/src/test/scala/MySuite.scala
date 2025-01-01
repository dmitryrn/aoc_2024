import scala.collection.mutable

// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html

class MySuite extends munit.FunSuite {
  test("parse should correctly parse a simple 2x2 matrix") {
    val input = """12
                  |34""".stripMargin
    val expected = Vector(Vector('1', '2'), Vector('3', '4'))
    assert(parse(input) == expected)
  }

  test("parse should correctly parse a 3x3 matrix with varying characters") {
    val input = """abc
                  |def
                  |ghi""".stripMargin
    val expected = Vector(Vector('a', 'b', 'c'), Vector('d', 'e', 'f'), Vector('g', 'h', 'i'))
    assert(parse(input) == expected)
  }

  test("parse should handle a matrix with leading and trailing whitespace") {
    val input = """  12  
                  |  34  """.stripMargin
    val expected = Vector(Vector('1', '2'), Vector('3', '4'))
    assert(parse(input) == expected)
  }

  test("parse should handle a matrix with empty rows") {
    val input = """12
                  |
                  |34""".stripMargin
    val expected = Vector(Vector('1', '2'), Vector.empty[Char], Vector('3', '4'))
    assert(parse(input) == expected)
  }

  test("travel") {
    val input = """
AAAA
BBCD
BBCC
EEEC
"""
    val m = parse(input)
    val visited = mutable.Set[(Int, Int)]()

    val (x, y) = (0, 0)

    val result = travel(m, visited, (x, y), m(y)(x))
    assertEquals(result, Some((10, 4)))
  }

  test("soln") {
    val input = """
AAAA
BBCD
BBCC
EEEC
"""

    val result = soln(input)
    assertEquals(result, 140)
  }
}
