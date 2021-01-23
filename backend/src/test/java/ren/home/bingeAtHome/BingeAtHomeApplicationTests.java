package ren.home.bingeAtHome;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.controller.ImageController;
import ren.home.bingeAtHome.controller.MetadataController;
import ren.home.bingeAtHome.controller.VideoController;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class BingeAtHomeApplicationTests {

    @Autowired
    private VideoController videoController;
    @Autowired
    private ImageController imageController;
    @Autowired
    private MetadataController metadataController;

    @Test
    public void contextLoads() {
        assertThat(videoController).isNotNull();
        assertThat(imageController).isNotNull();
        assertThat(metadataController).isNotNull();
    }

}
