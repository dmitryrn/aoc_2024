import scala.collection.mutable

def soln(str: String): Int = {
  val m = parse(str)

  val visited = mutable.Set[(Int, Int)]()

  val coordinates = for {
    y <- 0 until m.length
    x <- 0 until m(0).length
  } yield (x, y)

  val sum = coordinates
    .map{(coords) => 
      val (x, y) = coords
      val char = m(y)(x)
      travel(m = m, visited = visited, coord = (x, y), char = char) match {
        case None => None
        case Some(perimeter, area) => Some(perimeter * area)
      }
    }
    .filter(_.isDefined)
    .map(_.get)
    .sum

  sum
}

def safeGet[T](vec: Vector[T], i: Int): Option[T] = {
  if (i >= 0 && i < vec.length) Some(vec(i))
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
      val item = safeGet(m, y).flatMap(safeGet(_, x))

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
