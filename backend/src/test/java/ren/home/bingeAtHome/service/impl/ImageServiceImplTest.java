package ren.home.bingeAtHome.service.impl;

import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.service.exception.ImageMissingException;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class ImageServiceImplTest {

    private static final String EXT = ".webp";

    @Mock
    private ImageDao imageDao;

    @InjectMocks
    private ImageServiceImpl imageService;

    @Test
    void getPosterImage_whenExisting_thenReturnPath() throws Exception {
        String testFile = "best_mp4_for_test.mp4";
        String image = testFile + EXT;
        Path path = Paths.get(image);

        Mockito.when(imageDao.readImage(image)).thenReturn(path);

        assertThat(imageService.getPosterImage(testFile)).isEqualTo(path);
    }

    @Test
    void getPosterImage_whenNotExisting_thenException() throws Exception {
        String notExistsName = "not_exists.mp4";
        String missingImage = notExistsName + EXT;

        Mockito.when(imageDao.readImage(missingImage)).thenThrow(new IOException());

        assertThatThrownBy(() -> imageService.getPosterImage(notExistsName)).isInstanceOf(ImageMissingException.class);
    }
}