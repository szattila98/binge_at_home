package ren.home.bingeAtHome.config;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;
import ren.home.bingeAtHome.config.interceptor.StreamRequestLoggingInterceptor;

@Configuration
public class RequestConfig implements WebMvcConfigurer {

    private final StreamRequestLoggingInterceptor interceptor;

    @Autowired
    public RequestConfig(StreamRequestLoggingInterceptor interceptor) {
        this.interceptor = interceptor;
    }

    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(interceptor).addPathPatterns("/api/video/**");

    }
}
