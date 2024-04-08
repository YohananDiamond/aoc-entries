(def line-peg
  (peg/compile
    ~{:main (* (some (+ (/ (<- :d+) ,|['number $])
                        (/ (<- (some :dot)) ,|[:dot $])
                        (/ (<- (some :symbol)) ,|[:sym $])
                        ))
               "\n")
      :dot "."
      :symbol (if-not (+ :d :dot "\n") 1)
      }
    ))

(defn line-to-coordinates [line]
  (def arr @[])
  (var i 0)

  (each elem line
    (def len (let [[_ str] elem]
               (length str)))

    (array/push arr @{:sig elem :str-range [i (+ i len)]})
    (+= i len))
  arr
  )

(defn main [& args]
  (def grid @[])
  (loop [line :iterate (:read stdin :line)]
    (as-> line x
          (peg/match line-peg x)
          (line-to-coordinates x)
          (array/push grid x)
          ))

  (defn element-at [row pos]
    (find |(let [{:str-range [a b]} $]
             (and (>= pos a) (< pos b))) row)
    )

  (defn surrounding-elements
    [&keys {:grid grid :pos [y x] :str-x str-x}]

    (let [row (get grid y)
          above-row (get grid (dec y))
          below-row (get grid (inc y))]
      [# left-right
       (get row (dec x))
       (get row (inc x))
       # above
       ;(if above-row (map |(element-at above-row $) [str-x (dec str-x) (inc str-x)]))
       # below
       ;(if below-row (map |(element-at below-row $) [str-x (dec str-x) (inc str-x)]))
       ]))

  (def part-numbers @{})

  (loop [[y row] :pairs grid
         [x elem] :pairs row
         :let [{:sig [t _] :str-range [str-x _]} elem]
         :when (= t :sym)
         surr-elem :in (surrounding-elements :grid grid :pos [y x] :str-x str-x)
         :when surr-elem]
    (match surr-elem
      @{:sig ['number _]} (set (part-numbers surr-elem) true)
      _ nil)
    )

  (var sum 0)
  (loop [[{:sig ['number n-str]} _] :pairs part-numbers
         :let [n (scan-number n-str)]]
    (+= sum n))

  (print "Total: " sum)
  )
