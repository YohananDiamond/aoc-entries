(def input-peg
  (do
    (peg/compile
      ~{:main (any :row)
        :row (/ (* (some :tile) "\n") ,string)
        :tile (<- (set ".#"))
        }
      )))

(defn rows-to-universe [rows]
  (def galaxies @[])
  (loop [[y row] :pairs rows
         [x tile] :pairs row
         :when (= (string/format "%c" tile) "#")]
    (array/push galaxies @[x y])
    )
  @{:galaxies galaxies
    :size @[(length (in rows 0)) (length rows)]}
  )

(defn has-any-in-row [galaxies x]
  (def f (fn [[g-x _]] (= g-x x)))
  (-> (find f galaxies) (nil?) (not)))

(defn has-any-in-column [galaxies y]
  (def f (fn [[_ g-y]] (= g-y y)))
  (-> (find f galaxies) (nil?) (not)))

(defn expand-universe [uni]
  (var [size-x size-y] (in uni :size))
  (def {:galaxies galaxies} uni)

  # expand columns
  (var x 0)
  (while (< x size-x)
    (when (not (has-any-in-row galaxies x))
      # the expansion is the same as moving every galaxy to the right... to the right
      (loop [g :in galaxies
             :let [[g-x _] g]
             :when (> g-x x)]
        (set (g 0) (inc g-x))
        )
      (++ size-x) # since the expansion happened
      (++ x) # skip the new (empty) row
      )
    (++ x))

  # expand rows
  (var y 0)
  (while (< y size-y)
    (when (not (has-any-in-column galaxies y))
      # the expansion is the same as moving every galaxy to the right... to the right
      (loop [g :in galaxies
             :let [[_ g-y] g]
             :when (> g-y y)]
        (set (g 1) (inc g-y))
        )
      (++ size-y) # since the expansion happened
      (++ y) # skip the new (empty) row
      )
    (++ y))

  (set (uni :size) @[size-x size-y])
  )

(defn print-universe [uni]
  (def [size-x size-y] (in uni :size))

  (def rows @[])
  (loop [i :range [0 size-y]]
    (array/push rows (buffer (string/repeat "." size-x))))

  (loop [[x y] :in (in uni :galaxies)]
    (def row (in rows y))
    (set (row x) (in "#" 0)))

  (each row rows
    (printf "%s" row))
  )

(defn sum-of-shortest-paths [uni]
  (def {:galaxies galaxies} uni)
  (def calculated-pairs @{})
  (var sum 0)
  (loop [[i g1] :pairs galaxies
         [j g2] :pairs galaxies
         :when (not= i j)
         :let [[x1 y1] g1
               [x2 y2] g2
               unique-pair [x1 y1 x2 y2]
               reverse-pair [x2 y2 x1 y1]]
         # only calculate this if we didn't calculate it before (but reverse lol)
         :when (not (in calculated-pairs reverse-pair))]
    (def distance
         (+ (math/abs (- y2 y1))
            (math/abs (- x2 x1))))
    (+= sum distance)
    (set (calculated-pairs unique-pair) true)
    )
  sum
  )

(defn main [& args]
  (as-> (:read stdin :all) .v
        (peg/match input-peg .v)
        (rows-to-universe .v)
        (do (expand-universe .v) .v)
        (sum-of-shortest-paths .v)
        (pp .v)
        )
  )
