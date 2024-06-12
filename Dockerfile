FROM nginx:stable-alpine-perl
ARG DEBIAN_FRONTEND=noninteractive

# # Copy the build artifact from the build stage
# COPY --from=0 /app/dist /usr/share/nginx/html
RUN ls
COPY dist /usr/share/nginx/html

# # Copy the nginx configuration
COPY ./nginx.conf /etc/nginx/conf.d/default.conf

# # Expose the port
EXPOSE 8080

# # Start Nginx server
CMD ["nginx", "-g", "daemon off;"]

