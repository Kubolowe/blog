<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';

const blogs = ref([]);
const tags = ref([]);


async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = target.querySelector('#title').value;
  const content = target.querySelector('#content').value;

  await my_project_backend.add_blog(title, content, tags.value)
  
  await getBlogs()
}

async function getBlogs() {
  try {
    const tempBlogs = await my_project_backend.get_blogs();
    blogs.value = tempBlogs.map((blog) => ({
      ...blog,
      date: blog.date.toString(),
    }));
  } catch (error) {
    console.error("Error fetching blogs:", error);
    alert("Failed to load blogs.");
  }
}

function saveTag(e) {
  const newTag = e.target.value.trim();
  if (newTag && !tags.value.includes(newTag)) {
    tags.value.push(newTag); 
  }
  e.target.value = "";
}

function removeTag(id) {
  tags.value.splice(id, 1);
}

getBlogs();
</script>

<template>
  <main class="container mx-auto">
    <img src="/logo2.svg" alt="DFINITY logo" class="mx-auto mt-4" />
    <form
      class="grid gap-4 pb-4 mb-4 border-solid border-b-2 border-indigo-500"
      action="#"
      @submit="handleSubmit"
    >
      <div>
        <p class="text-black font-bold">Title</p>
        <input
          id="title"
          alt="Title"
          type="text"
          class="w-full rounded-full py-1 px-4 outline-none border-solid border-2 hover:border-indigo-700"
        />
      </div>
      <div>
        <p class="text-black font-bold">Content</p>
        <textarea
          id="content"
          alt="Content"
          type="text"
          class="w-full rounded-sm py-1 px-4 outline-none min-h-[100px] border-solid border-2 hover:border-indigo-700"
        />
      </div>
      <div>
        <p class="text-black font-bold">Tags</p>
        <input
          id="tags"
          alt="Tags"
          type="text"
          v-on:keyup.enter="saveTag"
          class="w-full rounded-full py-1 px-4 outline-none"
        />
        <div class="flex gap-1 flex-wrap my-2">
          <div
            v-for="(tag, id) in tags"
            :key="id"
            class="
              text-white 
              bg-indigo-400 
              rounded-3xl 
              py-1 
              px-4
              text-sm
              w-fit
            "
            @click="removeTag(id)"
          >
            {{ tag }}
          </div>
        </div>
      </div>
      <div class="flex justify-end">
        <button
          type="submit"
          class="text-white bg-indigo-400 rounded-3xl px-4 py-1 border-solid border-2 hover:border-indigo-700"
        >
          Click to add!
        </button>
      </div>
    </form>
    <div>
      <div v-for="blog in blogs" :key="blog.title" class="bg-white rounded-3xl p-4 mb-4">
        {{ new Date(blog.date / 1000000).toLocaleString() }}
        <h3 class="text-lg font-bold mb-2">{{ blog.title }}</h3>
        <p> {{ blog.content }}</p>
        <div class="flex gap-4 mt-2 text-indigo-500">
          <div
            v-for="tag in blog.tags"
            :key="tag"
            class="text-white bg-indigo-800 rounded-3xl py-1 px-4 w-fit"
          >
            <a>{{ tag }}</a>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>
