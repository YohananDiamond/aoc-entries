(defn main [& args]
  (var sum 0)
  (loop [line :iterate (:read stdin :line)]
    (as-> line x
          # get all digits
          (peg/match ~{:main (some (+ (<- :d) 1))} x)

          # get first and last digits
          (let [len (length x)]
            (assert (not (empty? x)))
            (string (in x 0) (in x (dec len))))

          (scan-number x) # parse int
          (+= sum x)
          ))
  (print "Total: " sum)
  )
