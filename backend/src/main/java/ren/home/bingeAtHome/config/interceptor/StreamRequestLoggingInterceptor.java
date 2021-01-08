package ren.home.bingeAtHome.config.interceptor;

import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.springframework.web.servlet.handler.HandlerInterceptorAdapter;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;

@Slf4j
@Component
public class StreamRequestLoggingInterceptor extends HandlerInterceptorAdapter {

    private class IpRecord {
        String ip;
        Date firstReceived;
        int timeout = 10;

        public IpRecord(String ip, Date firstReceived) {
            this.ip = ip;
            this.firstReceived = firstReceived;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            IpRecord ipRecord = (IpRecord) o;
            return ip.equals(ipRecord.ip);
        }
    }

    private static List<IpRecord> ips = new ArrayList<>();

    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) {
        IpRecord requestIp = new IpRecord(request.getRemoteAddr(), new Date());
        if (ips.contains(requestIp)) {
            Instant instant1 = requestIp.firstReceived.toInstant().truncatedTo(ChronoUnit.DAYS);
            Instant instant2 = new Date().toInstant().truncatedTo(ChronoUnit.DAYS);
            IpRecord storedIp = ips.get(ips.indexOf(requestIp));
            if (instant1.equals(instant2)) {
                storedIp.timeout--;
                if (storedIp.timeout <= 0) ips.remove(storedIp);
            } else {
                ips.remove(storedIp);
            }
        } else {
            log.info("Video range sent. Video name: {}, Range: {}, IP: {}", request.getRequestURI().split("/")[3],
                    request.getHeader("range"), requestIp.ip);
            ips.add(requestIp);
        }
        return true;
    }
}
