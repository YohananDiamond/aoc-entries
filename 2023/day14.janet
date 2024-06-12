(def input-peg
  (peg/compile
    ~{:main (some (* :section (any :s+)))
      :section (/ (some :row) ,tuple)
      :row (/ (* (some (<- (set "O#."))) "\n") ,array)
      }))

(defn move-rocks-up [graph]
  (def width (-> graph (in 0) (length)))
  (def height (-> graph (length)))

  (defn get-tile [x y]
    (-> graph (in y) (in x)))
  (defn set-tile [x y val]
    (-> graph (in y) (put x val)))

  (loop [x :range [0 width]
         i :range [0 height]
         :when (-> (get-tile x i) (= "."))]
    (var y (+ i 1))
    (while (< y height)
      (def t (get-tile x y))
      (when (= t "#")
        (break))
      (when (= t "O")
        (set-tile x i "O")
        (set-tile x y ".")
        (break))
      (++ y))
    ))

(defn print-graph [graph]
  (each row graph
    (each tile row
      (prinf "%s" tile))
    (prinf "\n")
    ))

(defn get-total-load [graph]
  (def width (-> graph (in 0) (length)))
  (def height (-> graph (length)))

  (defn get-tile [x y]
    (-> graph (in y) (in x)))

  (var sum 0)
  (loop [x :range [0 width]
         y :range [0 height]
         :when (-> (get-tile x y) (= "O"))]
    (+= sum (- height y)))
  sum)

(defn main [& args]
  (as-> (:read stdin :all) .v
        (peg/match input-peg .v)
        (in .v 0)
        (let [graph .v]
          (move-rocks-up graph)
          # (print-graph graph)
          (pp (get-total-load graph))
          )
  ))
