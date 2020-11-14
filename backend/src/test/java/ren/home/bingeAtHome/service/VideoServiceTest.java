package ren.home.bingeAtHome.service;

import org.apache.commons.io.FileUtils;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.impl.VideoServiceImpl;

import java.io.File;
import java.io.IOException;
import java.net.URISyntaxException;
import java.net.URL;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class VideoServiceTest {

    private static final String testFile = "best_mp4_for_test.mp4";

    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        URL resource = VideoServiceTest.class.getClassLoader().getResource(testFile);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File("./videos/" + testFile));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File("./videos"));
    }

    @Mock
    private VideoDao videoDao;

    @InjectMocks
    private final VideoService videoService = new VideoServiceImpl();

    @BeforeEach
    void setUpBeforeEach() {
        File file = new File("./videos/" + testFile);
        Mockito.when(videoDao.findAllVideos()).thenReturn(Lists.newArrayList(file));
    }

    @Test
    void getAllVideos_returnsTestMp4() {
        List<Video> videos = videoService.getAllVideos();
        assertThat(videos).hasSize(1);
        for (Video video : videos) {
            assertThat(video.getFileName()).isEqualTo(testFile);
        }
    }

    @Test
    void prepareContent() {
    }
}