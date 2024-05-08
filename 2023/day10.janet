(def input-peg
  (do
    (peg/compile
      ~{:main (any :row)
        :row (/ (* (some :tile) "\n") ,tuple)
        :tile (<- (set "|-LJ7F.S"))
        }
      )))

(defn tile-at [pipe-map x y]
  (-> pipe-map (in y) (in x)))

(defn find-starting-point [pipe-map]
  (label result
    (loop [[y row] :pairs pipe-map
           [x elem] :pairs row
           :when (= elem "S")]
      (return result [x y])))
  )

(def rotation-map
  {
   [:up "F"] :right
   [:up "7"] :left
   [:up "|"] :up
   [:up "S"] 'STOP

   [:right "7"] :down
   [:right "J"] :up
   [:right "-"] :right
   [:right "S"] 'STOP

   [:down "J"] :left
   [:down "L"] :right
   [:down "|"] :down
   [:down "S"] 'STOP

   [:left "L"] :up
   [:left "F"] :down
   [:left "-"] :left
   [:left "S"] 'STOP
   })

(defn next-tile
  "Looks up the next tile in the desired coordinates and direction. Returns the new coordinates, tile and direction if you can walk through said tile; nil if you can't."
  [pipe-map x y direction]

  (def [new-x new-y]
    (match direction
      :up [x (dec y)]
      :right [(inc x) y]
      :down [x (inc y)]
      :left [(dec x) y]
      ))

  (def new-tile (tile-at pipe-map new-x new-y))
  (if-let [new-dir (in rotation-map [direction new-tile])]
    [new-x new-y new-tile new-dir]
    nil)
  )

(def valid-directions [:up :right :down :left])

(defn rotate-clockwise [d]
  (def len (length valid-directions))
  (as-> (find-index |(= d $) valid-directions) .v
        (+ .v 1) (% .v len)
        (in valid-directions .v)))

(defn cycle-pipe-map
  "Cycle through the map clockwise and accumulate all tiles walked through in an array. The returned array has the starting point (S) as the first element."
  [pipe-map]

  (var [x y] (find-starting-point pipe-map))

  # get starting direction
  (var direction
    (as-> valid-directions .v
          (find |(-> (next-tile pipe-map x y $)
                     (nil?) (not)) .v)
          (or .v (error "No pathways found"))))

  (def trail @["S"])
  (forever
    # find the next tile (change direction if necessary)
    (def elem-info (next-tile pipe-map x y direction))
    (def [new-x new-y elem new-dir] elem-info)
    # (pp ['went direction '& 'found elem 'at [new-x new-y] 'new-dir-is new-dir])
    (set direction new-dir)
    (set x new-x)
    (set y new-y)

    (when (= elem "S")
      (break))
    (array/push trail elem)
    )

  trail
  )

(defn main [& args]
  # okay - my idea for this is to build an array with the entire path and then get the length of the middle
  (as-> (:read stdin :all) .v
        (peg/match input-peg .v)
        (cycle-pipe-map .v)
        (/ (length .v) 2)
        (pp .v)
        )
  )
