import { createRouter, createWebHistory } from "vue-router";
import ProjectsPage from "@/pages/ProjectsPage.vue";
import ProjectDetailPage from "@/pages/ProjectDetailPage.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "projects",
      component: ProjectsPage,
    },
    {
      path: "/project/:id",
      name: "project-detail",
      component: ProjectDetailPage,
      props: true,
    },
  ],
});

export default router;
