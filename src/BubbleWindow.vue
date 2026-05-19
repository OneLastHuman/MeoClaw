<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

// Envelope 前缀正则：匹配 [渠道名] 或 [渠道名 时间戳]
const ENVELOPE_REGEX = /^\[([^\]]+)\]\s*/;

const LONG_TEXT = `从前有座山，山里有座庙，庙里有个老和尚在给小和尚讲故事。老和尚说："从前有座山，山里有座庙，庙里有个老和尚在给小和尚讲故事。"小和尚问："然后呢？"老和尚继续说："从前有座山，山里有座庙，庙里有个老和尚在给小和尚讲故事。"小和尚又问："然后呢？"老和尚叹了口气："你这孩子，怎么就不明白呢？故事讲完了就是完了，哪有那么多然后啊！"小和尚若有所思地点点头："哦，那老和尚您继续讲吧。"老和尚无奈地摇摇头："罢了罢了，今天的故事就讲到这里吧。"小和尚眨眨眼："可是我还没听够呢。"老和尚摸了摸小和尚的头："那我们明天继续。"`;

const bubbleText = ref(LONG_TEXT);

let unlistenUpdate: (() => void) | null = null;

onMounted(async () => {
  // 尝试监听更新气泡内容事件（权限问题不影响静态显示）
  try {
    unlistenUpdate = await listen<{ text: string }>('bubble-update', (event) => {
      bubbleText.value = event.payload.text;
    });
  } catch (e) {
    console.warn('[Bubble] event listen failed, using static content');
  }
});

onUnmounted(() => {
  unlistenUpdate?.();
});

/**
 * 解析文本，提取 envelope 前缀和正文
 * 返回数组，每项包含 type（'envelope' | 'text'）和 content
 */
interface TextSegment {
  type: 'envelope' | 'text';
  content: string;
}

function parseTextWithEnvelope(text: string): TextSegment[] {
  const segments: TextSegment[] = [];
  const lines = text.split('\n');

  for (const line of lines) {
    const match = line.match(ENVELOPE_REGEX);
    if (match) {
      const envelope = match[0]; // 完整匹配包括方括号
      const rest = line.slice(match[0].length);
      if (envelope.trim()) {
        segments.push({ type: 'envelope', content: envelope });
      }
      if (rest.trim()) {
        segments.push({ type: 'text', content: rest });
      }
    } else {
      segments.push({ type: 'text', content: line });
    }
  }

  return segments;
}

const textSegments = computed(() => parseTextWithEnvelope(bubbleText.value));
</script>

<template>
  <div class="bubble-container">
    <div class="bubble">
      <div class="bubble-text">
        <template v-for="(seg, idx) in textSegments" :key="idx">
          <span
            :class="seg.type === 'envelope' ? 'bubble-envelope' : ''"
          >{{ seg.content }}</span>
          <br v-if="idx < textSegments.length - 1" />
        </template>
      </div>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
}

.bubble-container {
  width: 100%;
  height: 100%;
  overflow: visible;
  background: transparent;
  display: flex;
  justify-content: flex-end;
  align-items: flex-start;
  padding: 8px;
}

.bubble {
  width: 360px;
  height: 540px;
  background: rgba(255, 255, 255, 0.92);
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.2);
  padding: 12px 16px;
  overflow-y: auto;
  overflow-x: hidden;
}

.bubble-text {
  display: block;
  font-size: 14px;
  font-family: sans-serif;
  color: #333;
  line-height: 1.5;
  word-wrap: break-word;
  overflow-wrap: break-word;
  white-space: pre-wrap;
}

.bubble-envelope {
  color: #888;
  font-size: 12px;
  font-style: italic;
}
</style>
