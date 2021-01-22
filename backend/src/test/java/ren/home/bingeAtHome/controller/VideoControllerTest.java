package ren.home.bingeAtHome.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.core.io.UrlResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.*;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import ren.home.bingeAtHome.controller.handlers.WebRestControllerAdvice;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

import java.io.File;
import java.net.URL;
import java.util.*;

import static org.assertj.core.api.Assertions.assertThat;


@WebMvcTest(VideoController.class)
class VideoControllerTest {

    private final ObjectMapper objectMapper = new ObjectMapper();
    Video video = new Video("mock_vid.mp4", new Date(), new Date(), 60000L, 60000L, ".mp4", new Metadata());
    String notExistingFileName = "no_such.mp4";

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private VideoService service;

    @BeforeAll
    static void init() {
        ExternalConfigurationUtil.init();
    }

    @Test
    void listVideos_returnsCorrectArray() throws Exception {
        String uri = "/api/video";
        List<Video> videos = new ArrayList<>();
        videos.add(video);

        Mockito.when(service.getAllVideos()).thenReturn(videos);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON_VALUE)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        String content = mvcResult.getResponse().getContentAsString();
        assertThat(objectMapper.readValue(content, Video[].class)).isEqualTo(videos.toArray());
    }

    @Test
    void getOne_whenExisting_returnOkAndCorrectVid() throws Exception {
        String uri = "/api/video/" + video.getFileName();

        Mockito.when(service.getVideo(video.getFileName())).thenReturn(video);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        String content = mvcResult.getResponse().getContentAsString();
        assertThat(objectMapper.readValue(content, Video.class)).isEqualTo(video);
    }

    @Test
    void getOne_whenNotExisting_returnNotFoundAndErrorMsg() throws Exception {
        String uri = "/api/video/" + notExistingFileName;

        Mockito.when(service.getVideo(notExistingFileName)).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new VideoMissingException().getMessage() + "\"}");
    }

    @Test
    void streamVideo_whenExisting_returnsPartialContentAndRange() throws Exception {
        // copying resource as it's cannot be null
        String testFile = "best_mp4_for_test.mp4";
        String videoRoot = "./videos";
        String filepath = videoRoot + File.separator + testFile;
        URL resource = VideoControllerTest.class.getClassLoader().getResource(testFile);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File(filepath));

        // setting up request
        String uri = "/api/stream";
        HttpHeaders headers = new HttpHeaders();
        List<HttpRange> httpRanges = new ArrayList<>();
        long start = 0L, end = 20000L;
        httpRanges.add(HttpRange.createByteRange(start, end));
        headers.setRange(httpRanges);
        ResourceRegion resourceRegion = new ResourceRegion(new UrlResource("file:" + filepath), start, end);

        // mock
        Mockito.when(service.prepareContent(Mockito.eq(testFile), Mockito.any(HttpHeaders.class)))
                .thenReturn(ResponseEntity.status(HttpStatus.PARTIAL_CONTENT).body(resourceRegion));

        // sending request
        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).param("v", testFile)
                .headers(headers).accept(MediaType.APPLICATION_JSON)).andReturn();

        // assertions
        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(206);
        assertThat(mvcResult.getResponse().getContentLengthLong()).isEqualTo(resourceRegion.getCount());

        // cleaning up resource
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Test
    void streamVideo_whenNotExisting_returnsNotfoundAndErrorMsg() throws Exception {
        String uri = "/api/stream";

        Mockito.when(service.prepareContent(Mockito.eq(notExistingFileName), Mockito.any(HttpHeaders.class))).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).param("v", notExistingFileName)
                .headers(new HttpHeaders()).accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new VideoMissingException().getMessage() + "\"}");
    }

    @Test
    void getTrackInfo_whenExistingTrack_ThenCorrectInfoJson() throws Exception {
        String uri = "/api/track/info/" + video.getFileName();
        String testTrack = "best_mp4_for_test.mp4-ENG.vtt";
        String filepath = ExternalConfigurationUtil.trackStorePath + File.separator + testTrack;
        URL resource = VideoControllerTest.class.getClassLoader().getResource(testTrack);
        assert resource != null;
        File trackFile = new File(filepath);
        FileUtils.copyFile(new File(resource.toURI()), trackFile);
        Map<String, String> trackInfo = new HashMap<>();
        trackInfo.put("ENG", testTrack);


        Mockito.when(service.getTrackInfo(video.getFileName())).thenReturn(trackInfo);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(objectMapper.writeValueAsString(trackInfo));
    }

    @Test
    void getTrackInfo_whenNotExistingVideo_ThenNotFoundAndCorrectMsg() throws Exception {
        String uri = "/api/track/info/" + notExistingFileName;

        Mockito.when(service.getTrackInfo(notExistingFileName)).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new VideoMissingException().getMessage() + "\"}");
    }

    @Test
    void getTrack_whenExistingTrack_ThenCorrectVtt() throws Exception {
        String testTrack = "best_mp4_for_test.mp4-ENG.vtt";
        String uri = "/api/track/" + testTrack;
        String filepath = ExternalConfigurationUtil.trackStorePath + File.separator + testTrack;
        URL resource = VideoControllerTest.class.getClassLoader().getResource(testTrack);
        assert resource != null;
        File trackFile = new File(filepath);
        FileUtils.copyFile(new File(resource.toURI()), trackFile);


        Mockito.when(service.getTrack(testTrack)).thenReturn(trackFile);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentLengthLong()).isEqualTo(trackFile.length());
    }

    @Test
    void getTrack_whenNotExistingTrack_ThenNotFoundAndCorrectMsg() throws Exception {
        String uri = "/api/track/" + notExistingFileName;

        Mockito.when(service.getTrack(notExistingFileName)).thenThrow(new TrackMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new TrackMissingException().getMessage() + "\"}");

    }
}