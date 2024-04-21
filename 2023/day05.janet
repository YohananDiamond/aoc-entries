(def input-peg
  (do
    (defn parse-number-list [& numbers]
      (map scan-number numbers))

    (defn make-map-header [from to]
      {:from from :to to})

    (defn make-map-range [a b c]
      {:out-start a :in-start b :size c})

    (defn make-map [header & ranges]
      (def {:from from :to to} header)
      {:from from :to to :ranges ranges})

    (peg/compile
      ~{:main (* :s* "seeds:" :s* :number-list
                 :s* (any (* :map :s*)))
        :map (/ (* :map-header :s* (any (* :map-range :s*))) ,make-map)
        :map-header (/ (* (<- :a+) "-to-" (<- :a+) " map:" :s*) ,make-map-header)
        :map-range (/ (* :cap-number :s* :cap-number :s* :cap-number :s*) ,make-map-range)
        :number-list (/ (any (* :cap-number :s*)) ,tuple)

        :cap-number (/ (<- :d+) ,scan-number)
        }
      )))

(defn get-new-id [old-id ranges]
  (as->
    ranges x
    # find range that contains the id
    (find |(let [{:in-start i :size s} $]
             (and (>= old-id i)
                  (< old-id (+ i s)))
             )
          x)
    (if (nil? x)
      # just return the old id if we didn't find it
      old-id
      # return the mapped id if we found the desired range
      (let [{:out-start o :in-start i} x
             map-add (- o i)]
        (+ old-id map-add))
      )
    ))

(defn main [& args]
  (as-> (:read stdin :all) x
        (peg/match input-peg x)
        (let [[seeds & maps] x
              ids @{}]
          (set (ids "seed") seeds) # "seed", not "seeds".
          (each {:from from :to to :ranges ranges} maps
            (let [old-ids (in ids from)
                  new-ids (map |(get-new-id $ ranges) old-ids)]
              (set (ids to) new-ids)
              )
            )
          (in ids "location")
          )
        (min ;x)
        (pp x)
        )
  )
