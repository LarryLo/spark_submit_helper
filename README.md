# spark_submit_helper
A simple program, which submit the solution of subject, to test spark program at the Spark Competition in Taiwan.

* Pre-install
  ```
  curl https://sh.rustup.rs -sSf | sh
  ```
  
* Init Script
  ```
  # The script is used to help make needed directories and clone Code-Fight
  $ spark_submit_helper/bin/init.sh
  ```

* Install environment script
  ```
  # The script is used to install sbt
  $ spark_submit_helper/bin/install_env.sh
  ```

* Build Code
  ```
  $ cd spark_submit_helper/src
  $ cargo build
  ```
  
* How to use
  ```
  # Server Side
  $ cp spark_submit_helper/target/debug/spark_submit_helper ~/
  $ /bin/bash -c './spark_submit_helper > /tmp/spark_submit_helper.log 2>&1 &'
  
  
  # Client Side
  ## For user creation
  $ curl -XPOST -d '{"user":"GG"}' localhost:3000/create/user
  
  ## For subjects test
  ### python version
  $ curl -XPOST -d '{"user":"larry", "language":"python", "subject":"word_count", "solution":"def answer(data):\n    result = data.flatMap(lambda x: x.split(\" \")).map(lambda x: (x, 1)).reduceByKey(lambda x, y: x + y).sortBy(lambda x : x[0]).sortBy(lambda x: x[1],  ascending=False).collect()\n    return result"}'  localhost:3000/submit
  
  ### scala version
  $ curl -XPOST -d '{"user":"larry", "language":"scala", "subject":"word_count", "solution":"package org.sparktw.codefight\nimport org.apache.spark.rdd.RDD\nobject Solution {  def answer(data: RDD[String]): RDD[(String, Int)] = {    val result = data.flatMap(line => line.split(\" \")).map(word => (word, 1)).reduceByKey(_ + _).sortBy(x => x._1).sortBy(x => x._1, false)\n    result  }}"}'  localhost:3000/submit
  ```

* Response
  ```
  ## For user creation
  ### Success
  {"response_code":0,"response_message":"Create GG subjects directories successfully."}
  
  ### Syntax Error
  {"response_code":2,"response_message":"Fail messages."}
  
  ## For subjects test
  ### Success
  {"response_code":0,"response_message":"pass","metrics":{"total":3,"error":0,"success":3}}
  
  ### Syntax Error
  {"response_code":1,"response_message":"syntax error","metrics":{"total":127,"error":127,"success":0}}
  
  ### Fail
  {"response_code":2,"response_message":"fail","metrics":{"total":0,"error":1,"success":0}}
  ```
