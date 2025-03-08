class WebTracker {
  website_id;
  server_url;
  // 页面基本信息
  capturePageData() {
    return {
      url: window.location.href, // 当前页面完整URL
      domian: window.location.hostname, // 域名
      path: window.location.pathname, // 页面路径
      title: document.title, // 页面标题
      referrer: document.referrer, // 来源页面
      timestamp: Date.now(), // 访问时间戳
    };
  }

  // 设备和浏览器信息
  captureDeviceData() {
    return {
      userAgent: navigator.userAgent, // 浏览器信息
      language: navigator.language, // 语言
      screenWidth: window.screen.width, // 屏幕宽度
      screenHeight: window.screen.height, // 屏幕高度
      devicePixelRatio: window.devicePixelRatio, // 像素密度
      platform: navigator.platform, // 操作系统
    };
  }

  // 性能数据
  capturePerformanceData() {
    const navigationTiming = window.performance.getEntries()[0];
    const data = {
      totalLoadTime: navigationTiming.duration,
      domLoadTime:
        navigationTiming.domContentLoadedEventEnd - navigationTiming.startTime,
      redrectTime:
        navigationTiming.redirectEnd - navigationTiming.redirectStart,
      dnsTime:
        navigationTiming.domainLookupEnd - navigationTiming.domainLookupStart,
      tcpTime: navigationTiming.connectEnd - navigationTiming.connectStart,
    };

    return data;
  }

  // 用户交互追踪
  captureInteractionData(eventType, additionalInfo = {}) {
    return {
      eventType: eventType, // 事件类型（点击、滚动等）
      targetElement: {
        tagName: event.target.tagName, // 元素标签
        className: event.target.className, // 元素类名
        id: event.target.id, // 元素ID
      },
      timestamp: Date.now(),
      ...additionalInfo, // 额外的交互信息
    };
  }

  // 位置和地理信息
  async captureLocationData() {
    try {
      // 通过 IP 获取大致位置
      const response = await fetch("https://ipapi.co/json/");
      const locationData = await response.json();
      return {
        ip: locationData.ip,
        country: locationData.country_name,
        city: locationData.city,
        latitude: locationData.latitude,
        longitude: locationData.longitude,
      };
    } catch (error) {
      // 处理获取位置信息失败的情况
      return {};
    }
  }

  generateUniqueId() {
    return (
      Math.random().toString(36).substring(2, 15) +
      Math.random().toString(36).substring(2, 15)
    );
  }

  // 隐私保护标识符
  generateVisitorId() {
    // 生成唯一但可追踪的访问者ID
    const existingId = localStorage.getItem("visitor_id");
    if (existingId) return existingId;
    const newId = this.generateUniqueId();
    localStorage.setItem("visitor_id", newId);
    return newId;
  }

  // 数据发送方法
  sendTrackingData(data) {
    // 异步发送数据到服务器
    fetch(`${this.server_url}/api/track`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        visitorId: this.generateVisitorId(),
        ...data,
      }),
    });
  }

  // 初始化追踪
  async init() {
    const { currentScript, referrer } = document;
    const attr = currentScript.getAttribute.bind(currentScript);
    this.website_id = attr("data-website-id");
    this.server_url = attr("data-server-url");
    // 添加交互事件监听
    // document.addEventListener("click", (event) => {
    //   this.sendTrackingData({
    //     type: "interaction",
    //     interactionData: this.captureInteractionData("click", event),
    //   });
    // });

    // 可选：滚动深度追踪
    // window.addEventListener(
    //   "scroll",
    //   this.throttle(() => {
    //     this.sendTrackingData({
    //       type: "scroll",
    //       scrollDepth: this.calculateScrollDepth(),
    //     });
    //   }, 500)
    // );
  }

  // 工具方法：节流
  throttle(func, limit) {
    let inThrottle;
    return function () {
      const args = arguments;
      const context = this;
      if (!inThrottle) {
        func.apply(context, args);
        inThrottle = true;
        setTimeout(() => (inThrottle = false), limit);
      }
    };
  }

  // 计算滚动深度
  calculateScrollDepth() {
    const scrollTop = window.scrollY;
    const windowHeight = window.innerHeight;
    const documentHeight = document.documentElement.scrollHeight;

    return Math.round((scrollTop / (documentHeight - windowHeight)) * 100);
  }
}

let tracker = new WebTracker();
tracker.init();

(function () {
  const originalPushState = window.history.pushState;

  window.history.pushState = function () {
    originalPushState.apply(this, arguments);
    trackPageView();
  };

  function trackPageView() {
    tracker.sendTrackingData({
      websiteId: parseInt(tracker.website_id),
      type: "pageview",
      pageData: tracker.capturePageData(),
      deviceData: tracker.captureDeviceData(),
      performanceData: tracker.capturePerformanceData(),
      // locationData: await this.captureLocationData(),
    });
  }

  trackPageView();
})();
