# spark_submit_helper
A simple program which submit solution to test the Spark Competition in Taiwan 

* Pre-install
  * Rust ~1.17.0
  * Code-Fight [https://github.com/TaiwanSparkUserGroup/Code-Fight]
  
* Build Code
  ```
  $ cd spark_submit_helper/src
  $ cargo build
  ```
  
* How to use
  ```
  $ cp spark_submit_helper/target/debug/spark_submit_helper ~/
  $ spark_submit_helper -l python -s word_count
  ```
