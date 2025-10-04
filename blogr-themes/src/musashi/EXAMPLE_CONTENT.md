# Musashi Theme - Content Structure Guide

This theme requires your content to be structured in a specific way in your `content.md` file.
Below is a complete example with placeholder content that you can copy and customize.

---

## Complete Example content.md

```yaml
---
title: "Miyamoto Musashi"
description: "Developer, designer, and lifelong learner on the path of mastery"
author: "Miyamoto Musashi"
theme: "musashi"
theme_config:
  enable_animations: true
  show_brush_strokes: true
  zen_mode: false
  primary_color: "#2b2b2b"
  background_color: "#fefefe"
---

sections:
  about:
    title: "The Way"
    quote:
      text: "The way is in training. Become acquainted with every art."
      author: "Book of Five Rings"
    content: |
      <p>I walk the path of continuous improvement, where every day presents an opportunity
      to refine my craft. Like ink on paper, each experience leaves its mark, shaping who
      I am and what I create.</p>

      <p>My journey is guided by curiosity, discipline, and the pursuit of mastery.
      I believe in the power of focused work, thoughtful design, and code that speaks
      for itself.</p>
    principles:
      - "Discipline"
      - "Mastery"
      - "Simplicity"

  skills:
    title: "Expertise"
    items:
      - title: "Full-Stack Development"
        description: "Building robust applications with modern technologies and best practices"
        tags:
          - "Rust"
          - "TypeScript"
          - "React"
          - "Node.js"

      - title: "System Design"
        description: "Architecting scalable, maintainable systems that stand the test of time"
        tags:
          - "Microservices"
          - "APIs"
          - "Databases"
          - "Cloud"

      - title: "Product & Design"
        description: "Creating intuitive experiences that balance beauty with functionality"
        tags:
          - "UI/UX"
          - "Product Strategy"
          - "Design Systems"

  projects:
    title: "Selected Work"
    items:
      - title: "Zen Task Manager"
        status: "Live"
        description: "A minimalist task management system built with focus and flow in mind. Features a clean interface and powerful keyboard shortcuts."
        tech:
          - "React"
          - "TypeScript"
          - "Tailwind CSS"
        link: "https://github.com/yourusername/zen-tasks"

      - title: "Ink & Paper"
        status: "In Progress"
        description: "A note-taking application inspired by traditional pen and paper, bringing digital convenience to analog simplicity."
        tech:
          - "Rust"
          - "Tauri"
          - "SQLite"
        link: "https://github.com/yourusername/ink-paper"

      - title: "Haiku Compiler"
        status: "Complete"
        description: "An experimental programming language where every program reads like poetry. A meditation on code as art."
        tech:
          - "Rust"
          - "LLVM"
          - "Parser Combinators"
        link: "https://github.com/yourusername/haiku-lang"

  contact:
    title: "Connect"
    text: "Interested in collaborating or just want to chat? I'm always open to interesting conversations and new opportunities."
    email: "musashi@example.com"
    social:
      github: "https://github.com/yourusername"
      twitter: "https://twitter.com/yourusername"
      linkedin: "https://linkedin.com/in/yourusername"
```

---

## Section Breakdown

### Hero Section (Required)
```yaml
title: "Your Name"
description: "Your tagline or philosophy"
author: "Your Name"
```
These appear at the top of your page.

### About Section (Optional)
```yaml
sections:
  about:
    title: "Section Title"
    quote:
      text: "An inspirational quote"
      author: "Quote Author"
    content: |
      <p>Your bio paragraph one.</p>
      <p>Your bio paragraph two.</p>
    principles:
      - "Value 1"
      - "Value 2"
      - "Value 3"
```

### Skills Section (Optional)
```yaml
sections:
  skills:
    title: "Skills" # or "Expertise", "What I Do", etc.
    items:
      - title: "Skill Name"
        description: "Description of this skill area"
        tags:
          - "Technology 1"
          - "Technology 2"
```

### Projects Section (Optional)
```yaml
sections:
  projects:
    title: "Projects" # or "Work", "Portfolio", etc.
    items:
      - title: "Project Name"
        status: "Complete" # or "Live", "In Progress", etc.
        description: "What this project is about"
        tech:
          - "Tech 1"
          - "Tech 2"
        link: "https://github.com/username/project" # optional
```

### Contact Section (Optional)
```yaml
sections:
  contact:
    title: "Connect"
    text: "Your contact message"
    email: "your.email@example.com"
    social:
      github: "https://github.com/username"     # optional
      twitter: "https://twitter.com/username"   # optional
      linkedin: "https://linkedin.com/in/username" # optional
```

---

## Theme Configuration

```yaml
theme_config:
  enable_animations: true          # Smooth scroll and fade effects
  show_brush_strokes: true         # Decorative ink brush backgrounds
  zen_mode: false                  # Ultra-minimal mode
  primary_color: "#2b2b2b"        # Soft ink black
  background_color: "#fefefe"      # Paper white
  text_color: "#3a3a3a"           # Soft black
  secondary_text_color: "#6b6b6b" # Slate gray
  accent_color: "#4a4a4a"         # Charcoal
```

---

## Tips for Best Results

1. **Keep content meaningful** - Every word should serve a purpose
2. **Use proper HTML** - Wrap paragraphs in `<p>` tags in the content field
3. **Balance sections** - 3 skills and 3-5 projects work well visually
4. **Quote wisely** - Choose quotes that reflect your philosophy
5. **Embrace whitespace** - The theme is designed to breathe
6. **Optional sections** - Include only the sections you want
7. **Status badges** - Use clear, concise status labels like "Live", "Complete", "Ongoing"

---

## Minimal Example

If you prefer a simpler approach, here's a minimal configuration:

```yaml
---
title: "Your Name"
description: "What you do"
author: "Your Name"
theme: "musashi"
---

sections:
  about:
    content: |
      <p>A brief introduction about yourself.</p>

  contact:
    email: "you@example.com"
```

That's all you need to get started! The theme will gracefully handle any sections you omit.
