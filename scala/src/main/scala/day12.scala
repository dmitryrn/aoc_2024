import scala.collection.mutable

def soln(str: String): Int = {
  val m = parse(str)

  val visited = mutable.Set[(Int, Int)]()

  val coordinates = for {
    y <- 0 until m.length
    x <- 0 until m(0).length
  } yield (x, y)

  val sum = coordinates
    .flatMap{(coords) => // flatmap filters out Nones and unwraps Somes
      val (x, y) = coords
      val char = m(y)(x)
      travel(m = m, visited = visited, coord = (x, y), char = char) match {
        case None => None
        case Some(perimeter, area) => Some(perimeter * area)
      }
    }
    .sum

  sum
}

def safeGet[T](vec: Vector[Vector[T]], y: Int, x: Int): Option[T] = {
  if (y >= 0 && y < vec.length && x >= 0 && x < vec(y).length) Some(vec(y)(x))
  else None
}

def travel(
  m: Vector[Vector[Char]], 
  visited: mutable.Set[(Int, Int)],
  coord: (Int, Int),
  char: Char,
): Option[(Int, Int)] = {
  if visited.contains(coord)
    then None
  else {
    visited.add(coord)

    var perimeter = 0
    var area = 1

    var toVisit = List[(Int, Int)]()
    val (x, y) = coord
    for ((x, y) <- Vector((x,y-1), (x,y+1), (x-1,y), (x+1, y))) {
      val item = safeGet(m, y, x)

      item match 
        case None => perimeter += 1
        case Some(itemChar) =>
          if itemChar != char
            then perimeter += 1
          else 
            toVisit = toVisit.appended((x, y))
    }

    toVisit
      .flatMap{(x, y) => travel(m = m, visited = visited, coord = (x, y), char = char)}
      .foreach{(p, a) => { perimeter += p; area += a }}

    Some(perimeter, area)
  }
}

def parse(str: String): Vector[Vector[Char]] = str.trim.split('\n').map(_.trim.toVector).toVector
