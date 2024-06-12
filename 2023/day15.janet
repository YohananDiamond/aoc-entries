(def input-peg
  (peg/compile
    ~{:main (* (some (* :str "," (any "\n"))) (? :str))
      :str (<- (some (if-not (set "\n,") 1)))
      }))

(defn hash-string [str]
  (var val 0)
  (each c str
    (set val (-> val (+ c) (* 17) (% 256)))
    )
  val
  )

(defn main [& args]
  (as-> (:read stdin :all) .v
        (peg/match input-peg .v)
        (let [steps .v]
          (var total 0)
          (each s steps
            (def h (hash-string s))
            (+= total h))
          (pp total))
  ))
