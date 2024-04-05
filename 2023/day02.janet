(defn parse-dice [amount color]
  [(scan-number amount) (keyword color)])

(def line-peg
  (peg/compile
    ~{:main (* :id (any (* :dice :separator)) :dice "\n")
      :id (/ (* "Game " (<- :d+) ": ")
             ,scan-number)
      :dice (/ (* (<- :d+) " " (<- (+ "red" "green" "blue")))
               ,parse-dice)
      :separator (+ ", " "; ")
      }
    ))

(def MAX-DICES {:red 12
                :green 13
                :blue 14})

(defn is-valid-dice [[amount color]]
  (<= amount (in MAX-DICES color)))

(defn main [& args]
  (var sum 0)
  (loop [line :iterate (:read stdin :line)]
    (as->
      line x
      (peg/match line-peg x)

      (let [[id & dices] x]
        (when (all is-valid-dice dices)
          (+= sum id)
          ))
      ))
  (print "Total: " sum)
  )
