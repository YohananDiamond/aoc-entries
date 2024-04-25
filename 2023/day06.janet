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
      ~{:main (* :s* "Time:" :s* :number-list
                 :s* "Distance:" :s* :number-list)
        :number-list (/ (any (* :cap-number :s*)) ,tuple)
        :cap-number (/ (<- :d+) ,scan-number)
        }
      )))

(defn main [& args]
  (as-> (:read stdin :all) x
        (peg/match input-peg x)
        (let [[times distances] x
              race-count (length times)]
          (var prod 1)
          (loop [i :range [0 race-count]
                 :let [time (in times i)
                       record (in distances i)]]
            (var possibilities 0)
            (loop [n :range [1 time]
                   # see `notes06.txt` for info on how this calculation works
                   :when (< record (* n (- time n)))]
              (++ possibilities))
            (*= prod possibilities)
            )
          (pp prod)))
  )
