package ren.home.bingeAtHome;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import ren.home.bingeAtHome.util.ExternalConfig;

@SpringBootApplication
public class BingeAtHomeApplication {

	/**
	 * The entry point of the application.
	 *
	 * @param args the input arguments
	 */
	public static void main(String[] args) {
		ExternalConfig.init();
		SpringApplication.run(BingeAtHomeApplication.class, args);
	}

}
