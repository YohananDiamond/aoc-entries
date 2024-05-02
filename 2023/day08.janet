(def input-peg
  (do
    (defn make-node-mapping [a b c]
      {:node a :to [b c]})

    (peg/compile
      ~{:main (* :steps "\n\n" (/ (any :node-mapping) ,tuple))
        :steps (<- (some (set "LR")))
        :node (<- (* (repeat 3 (range "AZ"))))
        :node-mapping (/ (* :node :s* "=" :s* "(" :s*
                            :node :s* "," :s* :node :s*
                            ")" "\n")
                         ,make-node-mapping)
        }
      )))

(defn main [& args]
  (as-> (:read stdin :all) x
        (peg/match input-peg x)
        (let [[steps mappings] x
              graph @{}]
          (loop [{:node n :to t} :in mappings]
            (set (graph n) t))

          (var current-node "AAA")
          (var steps-i 0)
          (var steps-taken 0)
          (while (not= current-node "ZZZ")
            (def this-step (match (string/slice
                                    steps steps-i (inc steps-i))
                             "L" 0
                             "R" 1))
            (set current-node (-> graph
                                  (in current-node)
                                  (in this-step)))

            # increase variables
            (++ steps-taken)
            (set steps-i (% (inc steps-i) (length steps)))
            )
          (pp steps-taken)
          )
        )
  )
