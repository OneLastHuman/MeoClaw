(() => {
  const pet = document.getElementById("pet");
  const hero = document.querySelector(".hero");
  const stage = pet ? pet.parentElement : null;
  const copy = document.querySelector(".hero__copy");
  const hit = pet ? pet.querySelector(".pet__hit") : null;
  const sprite = document.getElementById("sprite");
  const speech = document.getElementById("speech");
  const home = document.getElementById("speechHome");
  const form = document.getElementById("askForm");
  const input = document.getElementById("askInput");
  const answer = document.getElementById("speechAnswer");
  const answerText = document.getElementById("answerText");
  const again = document.getElementById("askAgain");
  const sayHi = document.getElementById("sayHi");
  if (!pet || !hit || !sprite || !speech) return;

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)");
  const staticFrame = "assets/cat-static.png";
  const animatedFrame = "assets/cat-idle.webp";
  const setMotion = () => {
    sprite.src = reduced.matches ? staticFrame : animatedFrame;
  };
  setMotion();
  if (reduced.addEventListener) reduced.addEventListener("change", setMotion);

  const replies = [
    "网页版的我只会卖萌。装好以后，我能读文件、查资料、跑命令，还能在你干活的时候待在屏幕边上。",
    "现在只是演示。真正的我会连到你自己的 OpenClaw 或 Hermes，答案从你那边来，不经过我。",
    "在这儿聊不了多久。下载装好，双击我，就能一直聊了。"
  ];
  let replyIndex = 0;
  let timer = 0;
  let mode = "home";

  function show(which) {
    mode = which;
    home.hidden = which !== "home";
    form.hidden = which !== "ask";
    answer.hidden = which !== "working" && which !== "answer";
  }

  function openAsk() {
    window.clearTimeout(timer);
    show("ask");
    input.value = "";
    requestAnimationFrame(() => input.focus());
  }

  function closeAsk() {
    window.clearTimeout(timer);
    show("home");
    hit.focus({ preventScroll: true });
  }

  function startWorking() {
    window.clearTimeout(timer);
    show("working");
    answerText.innerHTML = '正在问后端<span class="dots" aria-hidden="true"><i></i><i></i><i></i></span>';
    answerText.setAttribute("aria-label", "正在问后端");
    again.hidden = true;
    timer = window.setTimeout(() => {
      const text = replies[replyIndex % replies.length];
      replyIndex += 1;
      show("answer");
      answerText.textContent = text;
      answerText.removeAttribute("aria-label");
      again.hidden = false;
    }, 1400);
  }

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const value = input.value.trim();
    if (!value) {
      closeAsk();
      return;
    }
    startWorking();
  });

  input.addEventListener("keydown", (event) => {
    if (event.key === "Escape") closeAsk();
  });

  again.addEventListener("click", openAsk);
  if (sayHi) sayHi.addEventListener("click", openAsk);

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && mode === "ask") closeAsk();
  });

  /* ---------------------------------------------------------------- drag */

  let tx = 0;
  let ty = 0;
  let drag = null;
  const isStacked = () => window.matchMedia("(max-width: 1000px)").matches;

  function apply() {
    pet.style.transform = `translate3d(${tx}px, ${ty}px, 0)`;
  }

  function unpeek() {
    pet.classList.remove("is-left", "is-right");
  }

  hit.addEventListener("pointerdown", (event) => {
    if (event.button !== 0) return;
    if (event.target.closest(".speech")) return;
    const rect = pet.getBoundingClientRect();
    drag = {
      id: event.pointerId,
      startX: event.clientX,
      startY: event.clientY,
      originX: tx,
      originY: ty,
      homeLeft: rect.left - tx,
      homeTop: rect.top - ty,
      width: rect.width,
      height: rect.height,
      moved: false,
      wasPeeked: pet.classList.contains("is-left") || pet.classList.contains("is-right")
    };
    hit.setPointerCapture(event.pointerId);
    pet.classList.add("is-dragging");
    unpeek();
    event.preventDefault();
  });

  hit.addEventListener("pointermove", (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    const dx = event.clientX - drag.startX;
    const dy = event.clientY - drag.startY;
    if (!drag.moved && Math.hypot(dx, dy) > 4) drag.moved = true;
    if (!drag.moved) return;

    const bounds = hero.getBoundingClientRect();
    const leftBound = isStacked()
      ? bounds.left + 6
      : copy.getBoundingClientRect().right + 12;
    const minX = leftBound - drag.homeLeft;
    const maxX = bounds.right - 6 - (drag.homeLeft + drag.width);
    const minY = stage.getBoundingClientRect().top + 4 - drag.homeTop;
    tx = Math.min(maxX, Math.max(minX, drag.originX + dx));
    ty = Math.min(0, Math.max(minY, drag.originY + dy));
    apply();
  });

  function endDrag(event) {
    if (!drag || event.pointerId !== drag.id) return;
    const bounds = hero.getBoundingClientRect();
    const left = drag.homeLeft + tx;
    const right = left + drag.width;
    pet.classList.remove("is-dragging");

    if (drag.moved) {
      if (isStacked() && left < bounds.left + 56) {
        tx = bounds.left - drag.homeLeft;
        pet.classList.add("is-left");
      } else if (right > bounds.right - 56) {
        tx = bounds.right - (drag.homeLeft + drag.width);
        pet.classList.add("is-right");
      }
      apply();
    } else if (drag.wasPeeked) {
      unpeek();
      tx = left < bounds.left + drag.width / 2
        ? bounds.left - drag.homeLeft
        : bounds.right - (drag.homeLeft + drag.width);
      apply();
    } else {
      openAsk();
    }
    drag = null;
  }

  hit.addEventListener("pointerup", endDrag);
  hit.addEventListener("pointercancel", (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    pet.classList.remove("is-dragging");
    drag = null;
  });

  hit.addEventListener("click", (event) => {
    if (event.detail === 0) openAsk();
  });

  /* --------------------------------------------------------------- extra */

  const copyBtn = document.getElementById("copyBtn");
  const sourceCode = document.getElementById("sourceCode");
  if (copyBtn && sourceCode) {
    copyBtn.addEventListener("click", async () => {
      try {
        await navigator.clipboard.writeText(sourceCode.textContent);
        copyBtn.textContent = "已复制";
        window.setTimeout(() => { copyBtn.textContent = "复制"; }, 1600);
      } catch {
        copyBtn.textContent = "复制失败";
        window.setTimeout(() => { copyBtn.textContent = "复制"; }, 1600);
      }
    });
  }

  let resizeTimer = 0;
  window.addEventListener("resize", () => {
    window.clearTimeout(resizeTimer);
    resizeTimer = window.setTimeout(() => {
      if (drag) return;
      tx = 0;
      ty = 0;
      unpeek();
      apply();
    }, 200);
  });
})();
