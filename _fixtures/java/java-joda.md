```java
// exemd-name: joda
// exemd-filename: HelloWorld
// exemd-deps: joda-time:joda-time;version=2.2
package hello;

import org.joda.time.LocalTime;

public class HelloWorld {
  public static void main(String[] args) {
    LocalTime currentTime = new LocalTime();
    System.out.println("The current local time is: " + currentTime);
  }
}
```