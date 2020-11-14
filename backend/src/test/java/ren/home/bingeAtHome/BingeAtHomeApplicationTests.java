package ren.home.bingeAtHome;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.controller.VideoController;

import java.io.File;
import java.io.IOException;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class BingeAtHomeApplicationTests {

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File("./config.properties"));
    }

    @Autowired
    private VideoController videoController;

    @Test
    public void contextLoads() {
        assertThat(videoController).isNotNull();
    }

}
