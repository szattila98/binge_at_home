package ren.home.homeflix.config;

import io.swagger.v3.oas.models.ExternalDocumentation;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.info.License;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/**
 * Open Api configuration for Swagger-ui generation.
 *
 * @author Attila Szőke
 */
@Configuration
public class OpenApiConfig {

    // TODO add security someday
    @Bean
    public OpenAPI customOpenAPI(@Value("${spring.application.name}") String appName,
                                 @Value("${spring.application.description}") String appDescription,
                                 @Value("${spring.application.version}") String appVersion) {
        return new OpenAPI()
                .info(new Info()
                        .title(appName)
                        .version(appVersion)
                        .description(appDescription)
                        .license(new License().name("GPL-3.0").url("http://www.gnu.org/licenses/gpl-3.0.en.html")))
                .externalDocs(new ExternalDocumentation()
                        .description("instructions on how to deploy this app")
                        .url("https://github.com/szattila98/binge_at_home/blob/main/README.md"));

    }
}
