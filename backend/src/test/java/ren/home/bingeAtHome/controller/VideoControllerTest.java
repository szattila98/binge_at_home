package ren.home.bingeAtHome.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.core.io.UrlResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpRange;
import org.springframework.http.MediaType;
import org.springframework.http.MediaTypeFactory;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import ren.home.bingeAtHome.controller.util.ErrorMsgJsonCreator;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.net.URL;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;


@WebMvcTest(VideoController.class)
class VideoControllerTest {

    private final ObjectMapper objectMapper = new ObjectMapper();
    private static final Video TEST_VIDEO = new Video(
            "mock_vid.mp4", new Date(), new Date(), 60000L, 60000L, ".mp4", new VideoMetadata());
    private static final String NOT_EXISTING_FILE = "no_such.file";
    private static final String VIDEO_REQUEST_URI = "/api/video/";
    private static final String STREAM_REQUEST_URI = "/api/stream";

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private VideoService service;

    @TempDir
    static File tempDir;

    @BeforeAll
    static void init() {
        ExternalConfig.test_init(tempDir);
    }

    @Test
    void listVideos_returnsCorrectArray() throws Exception {
        List<Video> videos = new ArrayList<>();
        videos.add(TEST_VIDEO);

        Mockito.when(service.getAllVideos()).thenReturn(videos);

        MvcResult mvcResult =
                mockMvc.perform(MockMvcRequestBuilders.get(VIDEO_REQUEST_URI)
                        .contentType(MediaType.APPLICATION_JSON)
                        .accept(MediaType.APPLICATION_JSON_VALUE)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        String content = mvcResult.getResponse().getContentAsString();
        assertThat(objectMapper.readValue(content, Video[].class)).isEqualTo(videos.toArray());
    }

    @Test
    void getOne_whenExisting_thenReturnOkAndCorrectVid() throws Exception {
        String uri = VIDEO_REQUEST_URI + TEST_VIDEO.getFileName();

        Mockito.when(service.getVideo(TEST_VIDEO.getFileName())).thenReturn(TEST_VIDEO);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri)
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        String content = mvcResult.getResponse().getContentAsString();
        assertThat(objectMapper.readValue(content, Video.class)).isEqualTo(TEST_VIDEO);
    }

    @Test
    void getOne_whenNotExisting_thenReturnNotFoundAndErrorMsg() throws Exception {
        String uri = VIDEO_REQUEST_URI + NOT_EXISTING_FILE;

        Mockito.when(service.getVideo(NOT_EXISTING_FILE)).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri)
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new VideoMissingException()));
    }

    @Test
    void streamVideo_whenExisting_thenReturnPartialContentAndRange() throws Exception {
        // copying resource as it's cannot be null
        String testVideo = "best_mp4_for_test.mp4";
        String filepath = ExternalConfig.VIDEO_STORE_PATH + File.separator + testVideo;
        URL resource = VideoControllerTest.class.getClassLoader().getResource(testVideo);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File(filepath));

        // setting up request
        HttpHeaders headers = new HttpHeaders();
        List<HttpRange> httpRanges = new ArrayList<>();
        long start = 0L, end = 20000L;
        httpRanges.add(HttpRange.createByteRange(start, end));
        headers.setRange(httpRanges);
        ResourceRegion resourceRegion = new ResourceRegion(
                new UrlResource("file:" + filepath), start, end);

        // mock
        Mockito.when(service.prepareContent(Mockito.eq(testVideo), Mockito.any(HttpHeaders.class)))
                .thenReturn(resourceRegion);

        // sending request
        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(STREAM_REQUEST_URI)
                .param("v", testVideo).headers(headers).accept(MediaType.APPLICATION_JSON))
                .andReturn();

        // assertions
        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(206);
        assertThat(mvcResult.getResponse().getContentLengthLong()).isEqualTo(resourceRegion.getCount());
        assertThat(mvcResult.getResponse().getContentType()).isEqualTo(MediaTypeFactory
                .getMediaType(resourceRegion.getResource()).orElse(MediaType.APPLICATION_OCTET_STREAM).toString());
    }

    @Test
    void streamVideo_whenNotExisting_thenReturnNotfoundAndErrorMsg() throws Exception {
        Mockito.when(service.prepareContent(
                Mockito.eq(NOT_EXISTING_FILE), Mockito.any(HttpHeaders.class)))
                .thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(STREAM_REQUEST_URI)
                .param("v", NOT_EXISTING_FILE)
                .headers(new HttpHeaders()).accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new VideoMissingException()));
    }
}