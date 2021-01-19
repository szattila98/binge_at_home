package ren.home.bingeAtHome.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import ren.home.bingeAtHome.controller.handlers.WebRestControllerAdvice;
import ren.home.bingeAtHome.service.ImageService;
import ren.home.bingeAtHome.service.exception.ImageMissingException;

import java.io.File;
import java.net.URL;
import java.nio.file.Paths;

import static org.assertj.core.api.Assertions.assertThat;

@WebMvcTest(ImageController.class)
class ImageControllerTest {

    private final ObjectMapper objectMapper = new ObjectMapper();
    String fileName = "best_mp4_for_test.mp4";
    String image = "best_mp4_for_test.mp4.webp";
    String uri = "/api/poster/" + fileName;

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private ImageService service;

    @Test
    void getPoster_whenExisting_thenReturnsCorrect() throws Exception {
        String videoRoot = "./videos";
        String filepath = videoRoot + File.separator + image;
        File file = new File(filepath);
        URL resource = VideoControllerTest.class.getClassLoader().getResource(image);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), file);

        Mockito.when(service.getPosterImage(fileName)).thenReturn(Paths.get(filepath));

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).accept(MediaType.ALL_VALUE)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentLengthLong()).isEqualTo(file.length());

        FileUtils.forceDelete(new File(videoRoot));
    }

    @Test
    void getPoster_whenNotExisting_thenNotFoundAndErrorMsg() throws Exception {
        Mockito.when(service.getPosterImage(fileName)).thenThrow(new ImageMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri).accept(MediaType.ALL_VALUE)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new ImageMissingException().getMessage() + "\"}");
    }
}