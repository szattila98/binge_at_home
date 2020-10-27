package ren.home.bingeAtHome;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class BingeAtHomeApplication {

	/**
	 * The entry point of the application.
	 *
	 * @param args the input arguments
	 */
	public static void main(String[] args) {
		// TODO check if properties are alright, store dir is dir, extensions are valid
		SpringApplication.run(BingeAtHomeApplication.class, args);
	}

}
