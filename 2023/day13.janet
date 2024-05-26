(def input-peg
  (peg/compile
    ~{:main (some (* :section (any :s+)))
      :section (/ (some :row) ,tuple)
      :row (/ (* (some (<- (set "#."))) "\n") ,tuple)
      }))

(defn columns-equal [section x1 x2]
  (let [h (length section)
        r (range 0 h)
        c1 (tuple ;(map |(-> section (in $) (in x1)) r))
        c2 (tuple ;(map |(-> section (in $) (in x2)) r))]
    (= c1 c2)
    ))

(defn find-reflection
  "Find the reflection in `section`, and return the sum of its (properly encoded) offsets (from x=0 if vertical, and from y=0 if horizontal). Returns 0 if none exist."
  [section]

  (var total 0)

  # search for a vertical line
  (label -vertical
    (loop [:let [width (length (in section 0))]
           line-x :range [1 width]
           :let [left-len line-x
                 right-len (- width line-x)
                 min-reflection-size (min left-len right-len)]]
      (label -l
        (loop [offset :range [1 (inc min-reflection-size)]
               :let [left-x (- line-x offset)
                     right-x (-> line-x (- 1) (+ offset))]]
          # if we find a difference, the reflection is not on line-y
          (unless (columns-equal section left-x right-x)
            (return -l nil)))
        # if we didn't find a difference, then it is here!
        (+= total line-x)
        (return -vertical))))

  # search for a horizontal line
  (when (= total 0)
    (label -horizontal
      (loop [:let [height (length section)]
             line-y :range [1 height]
             :let [top-len line-y
                   bot-len (- height line-y)
                   min-reflection-size (min top-len bot-len)]]
        (label -l
          (loop [offset :range [1 (inc min-reflection-size)]
                 :let [top-y (- line-y offset)
                       bot-y (-> line-y (- 1) (+ offset))]]
            # if we find a difference, the reflection is not on line-y
            (when (not= (in section top-y) (in section bot-y))
              (return -l nil)))
          # if we didn't find a difference, then it is here!
          (+= total (* 100 line-y))
          (return -horizontal)))))

    total)

(defn main [& args]
  (as-> (:read stdin :all) .v
        (peg/match input-peg .v)
        (do
          (var total 0)
          (loop [[i section] :pairs .v
                 :let [r (find-reflection section)]]
            (+= total r))
          total)
        (pp .v)
  ))
