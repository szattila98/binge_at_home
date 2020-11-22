package ren.home.bingeAtHome;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.controller.VideoController;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class BingeAtHomeApplicationTests {

    @Autowired
    private VideoController videoController;

    @Test
    public void contextLoads() {
        assertThat(videoController).isNotNull();
    }

}
