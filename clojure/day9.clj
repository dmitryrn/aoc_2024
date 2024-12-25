(ns day9
  (:require [clojure.test :refer :all]))

(defn my-range [i len]
  (map
    (fn [_] (if (odd? i) (quot i 2) -1))
    (range len)))

(defn expand [layout]
  (flatten (let [enumerated (map-indexed (fn [i v] [(inc i) v]) layout)]
            (map 
              (fn [[i v]] (my-range i v))
              enumerated))))
  

(defn compact [list]
  (let [
        enumerated (map-indexed (fn [i v] [i v]) list)
        [el-i el-v] (some (fn [[i, v]] (if (> v -1) [i v] nil)) (reverse enumerated))
        emp-i (some (fn [[i v]] (if (= v -1) i nil)) enumerated)]
    (if (> emp-i el-i)
      list
      (let [
            list (assoc (vec list) emp-i el-v)
            list (assoc (vec list) el-i -1)]
        (recur list)))))

(defn checksum [list]
  (let [enumerated (map-indexed (fn [i v] [i v]) (filter #(> % -1) list))]
    (reduce + (map (fn [[i v]] (* i v)) enumerated))))

(defn parse [line]
  (map #(Character/digit % 10) (seq line)))

(deftest test-expand
  (testing "expand"
    (is (= (-> "12345" parse expand) (parse "0..111....22222")))
    (is (= (-> "2333133121414131402" parse expand) (parse "00...111...2...333.44.5555.6666.777.888899")))))

(deftest test-compact1
  (testing "compact"
    (is (= 
         (-> [1 2 3 4 5] expand compact)
         [0 2 2 1 1 1 2 2 2 -1 -1 -1 -1 -1 -1]))
    (is (= 
         (-> [2 3 3 3 1 3 3 1 2 1 4 1 4 1 3 1 4 0 2] expand compact)
         (parse "0099811188827773336446555566..............")))))

(deftest test-spl-line
  (testing "spl-line"
    (is (= (parse "123") [1 2 3]))))

(deftest test-main
  (testing "main"
    (let [parsed (parse "2333133121414131402")
          expanded (expand parsed)
          compacted (compact expanded)
          checksum (checksum compacted)]
      (println "Parsed:" parsed)
      (println "Expanded:" expanded)
      (println "Compacted:" compacted)
      (println "Checksum:" checksum)
      (is (= checksum 1928)))))

(run-tests)
