package ren.home.bingeAtHome.controller;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;

/**
 * Hides all api routes, so users will only interact with the server single-page app.
 *
 * @author Attila Szőke
 */
@Controller
public class RouteController {

    /**
     * Redirects to the single page app.
     */
    @GetMapping("/**/{path:[^\\.]*}")
    public String redirect() {
        return "forward:/index.html";
    }
}
