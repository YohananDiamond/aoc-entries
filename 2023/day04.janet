(def line-peg
  (do
    (defn parse-number-list [& ls]
      (map scan-number ls))

    (peg/compile
      ~{:main (* :id :number-list "|" :number-list)
        :id (* "Card" :s+ :d+ ":" :s*)
        :number-list (/ (any (* :s* (<- :d+) :s*)) ,parse-number-list)
        }
      )))

(defn main [& args]
  (var sum 0)

  (loop [line :iterate (:read stdin :line)]
    (as->
      line x
      (peg/match line-peg x)
      (let [[winning have] x]
        (var points 0)
        (loop [n :in have
               :when (find |(= $ n) winning)]
          (set points (if (= points 0)
                        1
                        (* points 2)))
          )
        (+= sum points)
        )
      ))

  (print "Total: " sum)
  )
