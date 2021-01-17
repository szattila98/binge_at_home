package ren.home.bingeAtHome.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
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
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.service.MetadataService;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import static org.assertj.core.api.Assertions.assertThat;

@WebMvcTest(MetadataController.class)
public class MetadataControllerTest {

    private final ObjectMapper objectMapper = new ObjectMapper();
    String uri = "/api/metadata";
    String validFileName = "mock_video.mp4";
    String validMetadata = "{\"videoName\": \"Never\", \"description\": \"Gonna\", \"tags\": [\"Give\", \"You\", \"Up\"]}";
    String validDto = "{\"fileName\": \"" + validFileName + "\", " + validMetadata.replaceFirst("\\{", "").replaceFirst("}", "") + "}";

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private MetadataService service;

    @Test
    void saveMetadata_whenSaved_thenOkAndStringReturned() throws Exception {
        Mockito.when(service.saveMetadata(validFileName, objectMapper.readValue(validMetadata, Metadata.class))).thenReturn(validFileName);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.post(uri).content(validDto).accept(MediaType.APPLICATION_JSON)
                .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(validFileName);
    }

    @Test
    void saveMetadata_whenVideoNotFound_thenNotFound() throws Exception {
        Mockito.when(service.saveMetadata(validFileName, objectMapper.readValue(validMetadata, Metadata.class))).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.post(uri).content(validDto).accept(MediaType.APPLICATION_JSON)
                .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new VideoMissingException().getMessage() + "\"}");
    }

    @Test
    void saveMetadata_whenCannotBeSaved_thenInternalServerError() throws Exception {
        Mockito.when(service.saveMetadata(validFileName, objectMapper.readValue(validMetadata, Metadata.class))).thenThrow(new MetadataCannotBeSavedException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.post(uri).content(validDto).accept(MediaType.APPLICATION_JSON)
                .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(500);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo("{\"" + WebRestControllerAdvice.messageKey + "\":\"" + new MetadataCannotBeSavedException().getMessage() + "\"}");
    }

    @Test
    void saveMetadata_whenInvalidMetadataInput_thenBadRequest() throws Exception {
        String invalidFileName = "";
        String invalidMetadata = "{\"videoName\": \"\", \"description\": \"\", \"tags\": [], \"posterPath\": \"\", \"captionsPath\": []}";
        String invalidDto = "{\"fileName\": \"" + invalidFileName + "\", \"metadata\": " + invalidMetadata + "  }";

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.post(uri).content(invalidDto).accept(MediaType.APPLICATION_JSON)
                .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(400);
    }

}
