package ren.home.bingeAtHome.service;

import org.apache.commons.io.FileUtils;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.core.io.UrlResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpRange;
import org.springframework.http.ResponseEntity;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.service.impl.VideoServiceImpl;

import java.io.File;
import java.io.IOException;
import java.net.MalformedURLException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Paths;
import java.util.List;
import java.util.Objects;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class VideoServiceTest {

    private static final String testFile = "best_mp4_for_test.mp4";
    private static final String videoRoot = "./videos";
    private static final String notExistsName = "not_exists.mp4";

    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        URL resource = VideoServiceTest.class.getClassLoader().getResource(testFile);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File(videoRoot + "/" + testFile));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Mock
    private VideoDao videoDao;
    private MetadataDao metadataDao;

    @InjectMocks
    private final VideoService videoService = new VideoServiceImpl();

    @Test
    void getAllVideos_returnsTestMp4_notReturnsMissingFileInVideoList() throws IOException {
        File file = new File(videoRoot + "/" + testFile);
        File notExists = new File(videoRoot + "/" + notExistsName);

        Mockito.when(videoDao.findAllVideoFiles()).thenReturn(Lists.newArrayList(file, notExists));

        List<Video> videos = videoService.getAllVideos();
        assertThat(videos).hasSize(1);
        for (Video video : videos) {
            assertThat(video.getFileName()).isEqualTo(testFile);
        }
    }

    @Test
    void prepareContent_whenRangeNotNullAndCorrectSize_returnCorrectResponseEntity() throws MalformedURLException, VideoMissingException {
        int expectedStatus = 206;
        long rangeStart = 0;
        long rangeEnd = 5000;
        long rangeLength = 5001;
        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.setRange(Lists.list(HttpRange.createByteRange(rangeStart, rangeEnd)));
        UrlResource resource = new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile).toString());

        Mockito.when(videoDao.findResourceByName(testFile)).thenReturn(resource);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(testFile, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(resource);
        assertThat(re.getBody().getCount()).isEqualTo(rangeLength);
        assertThat(re.getBody().getPosition()).isEqualTo(rangeStart);
    }

    @Test
    void prepareContent_whenRangeNotNullAndTooLarge_returnCorrectRegion() throws MalformedURLException, VideoMissingException {
        int expectedStatus = 206;
        long rangeStart = 0;
        long rangeEnd = 1024 * 1024 + 1;
        long rangeLength = 1024 * 1024;
        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.setRange(Lists.list(HttpRange.createByteRange(rangeStart, rangeEnd)));
        UrlResource resource = new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile).toString());

        Mockito.when(videoDao.findResourceByName(testFile)).thenReturn(resource);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(testFile, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(resource);
        assertThat(re.getBody().getCount()).isEqualTo(rangeLength);
        assertThat(re.getBody().getPosition()).isEqualTo(rangeStart);
    }

    @Test
    void prepareContent_whenNullRange_returnPreDefinedRegion() throws IOException, VideoMissingException {
        int expectedStatus = 206;
        long maxRangeLength = 1024 * 1024;
        HttpHeaders httpHeaders = new HttpHeaders();
        UrlResource resource = new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile).toString());

        Mockito.when(videoDao.findResourceByName(testFile)).thenReturn(resource);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(testFile, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(resource);
        assertThat(re.getBody().getCount()).isEqualTo(maxRangeLength);
    }

    @Test
    void prepareContent_whenNotExistingVideo_throwException() throws MalformedURLException {
        Mockito.when(videoDao.findResourceByName(notExistsName)).thenThrow(new MalformedURLException());

        assertThatThrownBy(() -> videoService.prepareContent(notExistsName, null)).isInstanceOf(VideoMissingException.class);
    }
}